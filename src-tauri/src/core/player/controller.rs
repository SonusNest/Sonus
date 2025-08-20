use std::path::Path;
use std::time::Duration;
use crate::core::player::audio_backend::AudioBackend;
use crate::core::player::state::{PlaybackState, SharedState};

pub struct PlayerController {
    backend: AudioBackend,
    state: SharedState,
}

impl PlayerController {
    pub fn new(state: SharedState) -> anyhow::Result<Self> {
        let backend = AudioBackend::new(state.clone())?;
        Ok(Self { backend, state })
    }

    pub fn load_and_play<P: AsRef<Path>>(&mut self, path: P) -> anyhow::Result<()> {
        self.backend.load_and_play(path, Duration::ZERO)
    }

    pub fn play_from<P: AsRef<Path>>(&mut self, path: P, position: Duration) -> anyhow::Result<()> {
        self.backend.load_and_play(path, position)
    }

    pub fn pause(&mut self) { self.backend.pause(); }

    pub fn resume(&mut self) { self.backend.resume(); }

    pub fn stop(&mut self) { self.backend.stop(); }

    pub fn set_volume(&mut self, v: f32) { self.backend.set_volume(v); }

    pub fn seek(&mut self, pos: Duration) -> anyhow::Result<()> { self.backend.seek(pos) }

    pub fn snapshot(&self) -> (PlaybackState, f32, Duration, Option<Duration>, Option<String>) {
        let s = self.state.lock().unwrap_or_else(|e| e.into_inner());
        (
            s.playback_state(),
            s.volume(),
            s.current_position(),
            s.total_duration(),
            s.current_file().map(|s| s.to_string()),
        )
    }

    pub fn shutdown(&mut self) { self.backend.shutdown(); }
}
