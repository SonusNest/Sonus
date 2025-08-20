use std::{
    fs::File,
    path::Path,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use anyhow::Result;
use rodio::{OutputStream, OutputStreamBuilder, Sink, Source};
use symphonia::core::{
    audio::{SampleBuffer, SignalSpec},
    codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL},
    errors::Error as SymphoniaError,
    formats::{FormatOptions, FormatReader, SeekMode, SeekTo},
    io::{MediaSourceStream, MediaSourceStreamOptions, ReadOnlySource},
    meta::MetadataOptions,
    probe::Hint,
    units::{Time},
};

use crate::core::player::state::{PlaybackState, SharedState};

/// rodio 输出 + sink 生命周期，配合 symphonia 解码与精准 seek。
pub struct AudioBackend {
    _stream: OutputStream, // 保持流生命周期，防止输出被 drop
    sink: Sink,            // 用 _stream.mixer() 创建
    state: SharedState,
    progress: ProgressClock,
}

impl AudioBackend {
    pub fn new(state: SharedState) -> Result<Self> {
        // 创建默认输出流
        let stream = OutputStreamBuilder::open_default_stream()
            .map_err(|e| anyhow::anyhow!("open_default_stream failed: {e}"))?;

        // 用 mixer 创建 Sink
        let sink = Sink::connect_new(&stream.mixer());

        sink.set_volume(state.lock().unwrap_or_else(|e| e.into_inner()).volume());

        Ok(Self {
            _stream: stream,
            sink,
            state,
            progress: ProgressClock::new(),
        })
    }

    /// 加载并从指定位置播放（position 可为 0）
    pub fn load_and_play<P: AsRef<Path>>(&mut self, path: P, position: Duration) -> Result<()> {
        let path_buf = path.as_ref().to_path_buf();
        let built = SymphoniaSource::from_path_start(&path_buf, position)?;
        let total = built.total_duration;
        let start_pos = built.start_position;

        let volume = {
            let s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.volume()
        };

        self.sink.stop();
        self.sink = Sink::connect_new(&self._stream.mixer());
        self.sink.set_volume(volume);
        self.sink.append(built);
        self.sink.play();

        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.set_current_file(Some(path_buf.to_string_lossy().to_string()));
            s.set_total_duration(total);
            s.set_current_position(start_pos);
            s.set_playback_state(PlaybackState::Playing);
        }

        self.progress.stop();
        self.progress.start(start_pos, /*paused=*/ false, self.state.clone());

        Ok(())
    }

    pub fn pause(&mut self) {
        self.sink.pause();
        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.set_playback_state(PlaybackState::Paused);
        }
        self.progress.pause();
    }

    pub fn resume(&mut self) {
        self.sink.play();
        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.set_playback_state(PlaybackState::Playing);
        }
        self.progress.resume();
    }

    pub fn stop(&mut self) {
        self.progress.stop();
        self.sink.stop();
        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.set_playback_state(PlaybackState::Stopped);
            s.set_current_position(Duration::ZERO);
            s.set_current_file(None);
        }
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.sink.set_volume(volume);
        self.state.lock().unwrap_or_else(|e| e.into_inner()).set_volume(volume);
    }

    /// 通过重建 Source 来实现精准跳转（兼容大多数压缩/封装）
    pub fn seek(&mut self, position: Duration) -> Result<()> {
        let target_path = {
            let s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.current_file()
                .map(|p| p.to_string())
                .ok_or_else(|| anyhow::anyhow!("cannot seek: no current file loaded"))?
        };

        let built = SymphoniaSource::from_path_start(Path::new(&target_path), position)?;
        let total = built.total_duration;

        self.sink.stop();
        self.sink = Sink::connect_new(&self._stream.mixer());
        self.sink.set_volume(self.state.lock().unwrap_or_else(|e| e.into_inner()).volume());
        self.sink.append(built);

        if self.state.lock().unwrap_or_else(|e| e.into_inner()).is_paused() {
            self.sink.pause();
            self.progress.seek_and_pause(position);
        } else {
            self.sink.play();
            self.progress.seek_and_resume(position);
        }

        {
            let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
            s.set_total_duration(total);
            s.set_current_position(position);
        }

        Ok(())
    }

    pub fn shutdown(&mut self) {
        self.progress.stop();
        self.sink.stop();
        // 清理状态
        let mut s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        s.set_playback_state(PlaybackState::Stopped);
        s.set_current_position(Duration::ZERO);
        s.set_current_file(None);
    }
}

/* ====================== 进度时钟 ======================== */

struct ProgressClock {
    inner: Option<ProgressInner>,
}

struct ProgressInner {
    handle: std::thread::JoinHandle<()>,
    ctl: Arc<ProgressCtl>,
}

struct ProgressCtl {
    state: SharedState,
    base_pos: Mutex<Duration>,          // 最近一次 start/seek 的位置
    start_instant: Mutex<Option<Instant>>, // None 表示暂停
    paused: Mutex<bool>,
    quit: std::sync::atomic::AtomicBool,
}

impl ProgressClock {
    fn new() -> Self {
        Self { inner: None }
    }

    fn start(&mut self, pos: Duration, paused: bool, state: SharedState) {
        self.stop();

        let ctl = Arc::new(ProgressCtl {
            state,
            base_pos: Mutex::new(pos),
            start_instant: Mutex::new(Some(Instant::now())),
            paused: Mutex::new(paused),
            quit: std::sync::atomic::AtomicBool::new(false),
        });

        let ctl2 = ctl.clone();
        let handle = std::thread::spawn(move || {
            use std::sync::atomic::Ordering;
            let tick = Duration::from_millis(200);
            while !ctl2.quit.load(Ordering::Relaxed) {
                std::thread::sleep(tick);

                let paused = *ctl2.paused.lock().unwrap_or_else(|e| e.into_inner());
                if paused {
                    continue;
                }

                let since = ctl2
                    .start_instant
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .map(|inst| inst.elapsed())
                    .unwrap_or_default();
                let pos_now = *ctl2.base_pos.lock().unwrap_or_else(|e| e.into_inner()) + since;

                let mut s = ctl2.state.lock().unwrap_or_else(|e| e.into_inner());
                if let Some(total) = s.total_duration() {
                    let pos_now_clamped = pos_now.min(total);
                    s.set_current_position(pos_now_clamped);
                    if pos_now_clamped >= total && s.is_playing() {
                        s.set_playback_state(PlaybackState::Stopped);
                        ctl2.quit.store(true, Ordering::Relaxed);
                    }
                } else {
                    s.set_current_position(pos_now);
                }
            }
        });

        self.inner = Some(ProgressInner { handle, ctl });
    }

    fn pause(&self) {
        if let Some(inner) = &self.inner {
            let mut paused = inner.ctl.paused.lock().unwrap_or_else(|e| e.into_inner());
            if !*paused {
                // 缩短锁持有时间
                let elapsed = {
                    inner.ctl.start_instant.lock().unwrap_or_else(|e| e.into_inner())
                        .map(|inst| inst.elapsed())
                        .unwrap_or_default()
                };
                *inner.ctl.base_pos.lock().unwrap_or_else(|e| e.into_inner()) += elapsed;
                *inner.ctl.start_instant.lock().unwrap_or_else(|e| e.into_inner()) = None;
                *paused = true;
            }
        }
    }

    fn resume(&self) {
        if let Some(inner) = &self.inner {
            let mut paused = inner.ctl.paused.lock().unwrap_or_else(|e| e.into_inner());
            if *paused {
                *inner.ctl.start_instant.lock().unwrap_or_else(|e| e.into_inner()) = Some(Instant::now());
                *paused = false;
            }
        }
    }

    fn seek_and_pause(&self, pos: Duration) {
        if let Some(inner) = &self.inner {
            {
                let mut base_pos = inner.ctl.base_pos.lock().unwrap_or_else(|e| e.into_inner());
                *base_pos = pos;
            }
            {
                let mut start_instant = inner.ctl.start_instant.lock().unwrap_or_else(|e| e.into_inner());
                *start_instant = None;
            }
            {
                let mut paused = inner.ctl.paused.lock().unwrap_or_else(|e| e.into_inner());
                *paused = true;
            }
            {
                let mut s = inner.ctl.state.lock().unwrap_or_else(|e| e.into_inner());
                s.set_current_position(pos);
            }
        }
    }


    fn seek_and_resume(&self, pos: Duration) {
        if let Some(inner) = &self.inner {
            {
                let mut base_pos = inner.ctl.base_pos.lock().unwrap_or_else(|e| e.into_inner());
                *base_pos = pos;
            }
            {
                let mut start_instant = inner.ctl.start_instant.lock().unwrap_or_else(|e| e.into_inner());
                *start_instant = Some(Instant::now());
            }
            {
                let mut paused = inner.ctl.paused.lock().unwrap_or_else(|e| e.into_inner());
                *paused = false;
            }
            {
                let mut s = inner.ctl.state.lock().unwrap_or_else(|e| e.into_inner());
                s.set_current_position(pos);
            }
        }
    }


    fn stop(&mut self) {
        if let Some(inner) = self.inner.take() {
            use std::sync::atomic::Ordering;
            inner.ctl.quit.store(true, Ordering::Relaxed);
            let _ = inner.handle.join();
        }
    }
}

/* ====================== Symphonia Source ======================== */

/// 将 symphonia 的解码结果包装成 rodio::Source；支持从任意时间点开始。
struct SymphoniaSource {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,

    sample_rate: u32,
    channels_count: u16,
    signal_spec: SignalSpec,

    buf: Vec<f32>,
    buf_pos: usize,

    total_duration: Option<Duration>,
    start_position: Duration,
}

impl SymphoniaSource {
    pub fn from_path_start(path: &Path, start: Duration) -> Result<Self> {
        let file = File::open(path)?;
        let ro = ReadOnlySource::new(file);
        let mss = MediaSourceStream::new(Box::new(ro), MediaSourceStreamOptions::default());

        let mut hint = Hint::new();
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            hint.with_extension(ext);
        }

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &FormatOptions::default(), &MetadataOptions::default())?;

        let mut format = probed.format;

        let track = format
            .default_track()
            .ok_or_else(|| anyhow::anyhow!("no default track"))?;
        if track.codec_params.codec == CODEC_TYPE_NULL {
            return Err(anyhow::anyhow!("unsupported codec"));
        }

        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &DecoderOptions::default())?;

        let sample_rate = track
            .codec_params
            .sample_rate
            .ok_or_else(|| anyhow::anyhow!("missing sample_rate"))?;
        let channels = track
            .codec_params
            .channels
            .ok_or_else(|| anyhow::anyhow!("missing channels"))?;
        let channels_count = channels.count() as u16;

        let signal_spec = SignalSpec { rate: sample_rate, channels };

        let total_duration = calc_total_duration(&track.codec_params);

        let track_id = track.id;

        // seek 到目标起点
        let start_position = start;
        if start > Duration::ZERO {
            let time = time_from_duration(start);
            if let Some(tb) = track.codec_params.time_base {
                let ts = tb.calc_timestamp(time);
                format.seek(SeekMode::Accurate, SeekTo::TimeStamp { ts, track_id })?;
            } else {
                format.seek(
                    SeekMode::Accurate,
                    SeekTo::Time {
                        time,
                        track_id: Some(track_id),
                    },
                )?;
            }
        }

        Ok(Self {
            format,
            decoder,
            track_id,
            sample_rate,
            channels_count,
            signal_spec,
            buf: Vec::new(),
            buf_pos: 0,
            total_duration,
            start_position,
        })
    }

    #[inline]
    fn refill(&mut self) -> Result<(), SymphoniaError> {
        self.buf.clear();
        self.buf_pos = 0;

        loop {
            match self.format.next_packet() {
                Ok(packet) => {
                    if packet.track_id() != self.track_id {
                        continue;
                    }
                    let decoded = self.decoder.decode(&packet)?;

                    let spec = *decoded.spec();
                    let frames = decoded.frames();
                    if frames == 0 {
                        continue; // <- 这里跳过空帧
                    }

                    let mut sbuf = SampleBuffer::<f32>::new(frames as u64, spec);
                    sbuf.copy_interleaved_ref(decoded);
                    self.buf.extend_from_slice(sbuf.samples());

                    if !self.buf.is_empty() {
                        return Ok(());
                    }
                }
                Err(SymphoniaError::ResetRequired) => {
                    self.decoder.reset();
                }
                Err(err) => return Err(err),
            }
        }
    }
}

impl Iterator for SymphoniaSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.buf_pos >= self.buf.len() {
            if let Err(_) = self.refill() {
                return None;
            }
            if self.buf_pos >= self.buf.len() {
                return None;
            }
        }
        let s = self.buf[self.buf_pos];
        self.buf_pos += 1;
        Some(s)
    }
}

impl Source for SymphoniaSource {
    fn current_span_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        self.channels_count
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        self.total_duration
    }
}

/* ---------------- 辅助函数 ---------------- */

fn time_from_duration(dur: Duration) -> Time {
    let seconds = dur.as_secs(); // u64
    let frac_bits: u32 = 32;
    let frac_unit = 1u64 << frac_bits;
    let frac_u64 = ((dur.subsec_nanos() as u128 * frac_unit as u128) / 1_000_000_000u128) as u64;
    let frac = frac_u64 as f64; // 转成 f64

    Time {
        seconds,
        frac,
    }
}



fn calc_total_duration(params: &symphonia::core::codecs::CodecParameters) -> Option<Duration> {
    let n_frames = params.n_frames?;
    let tb = params.time_base?;
    let t = tb.calc_time(n_frames);

    let secs = t.seconds.max(0) as u64;
    let frac_secs = n_frames as f64 / params.sample_rate? as f64 - secs as f64;

    Some(Duration::from_secs(secs) + Duration::from_secs_f64(frac_secs))
}

