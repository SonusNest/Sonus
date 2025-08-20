#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use crate::core::player::audio_backend::AudioBackend;
    use crate::core::player::state::{PlaybackState, new_shared_state};

    fn approx_eq(d1: Duration, d2: Duration, epsilon: Duration) -> bool {
        let diff = if d1 > d2 { d1 - d2 } else { d2 - d1 };
        diff <= epsilon
    }

    #[test]
    fn test_audio_backend_full_flow() {
        let state = new_shared_state();
        let mut backend = AudioBackend::new(state.clone()).unwrap();

        let files = [
            r"D:\Music\music\suzume_no_tojimari.flac",
            r"D:\Music\music\suzume_no_tojimari.ogg",
            r"D:\Music\music\suzume_no_tojimari.wav",
            r"D:\Music\music\suzume_no_tojimari.mp3",
        ];

        for file in files {
            println!("=== Playing {} ===", file);
            backend.load_and_play(file, Duration::ZERO).unwrap();

            // 播放 2 秒
            std::thread::sleep(Duration::from_secs(2));
            assert_eq!(state.lock().unwrap().playback_state(), PlaybackState::Playing);

            // 暂停 1 秒
            backend.pause();
            std::thread::sleep(Duration::from_secs(1));
            assert_eq!(state.lock().unwrap().playback_state(), PlaybackState::Paused);

            // 继续播放 2 秒
            backend.resume();
            std::thread::sleep(Duration::from_secs(2));
            assert_eq!(state.lock().unwrap().playback_state(), PlaybackState::Playing);

            // 音量 30%，播放 2 秒
            backend.set_volume(0.3);
            std::thread::sleep(Duration::from_secs(2));
            assert!((state.lock().unwrap().volume() - 0.3).abs() < f32::EPSILON);

            // 音量恢复 100%，播放 1 秒
            backend.set_volume(1.0);
            std::thread::sleep(Duration::from_secs(1));
            assert!((state.lock().unwrap().volume() - 1.0).abs() < f32::EPSILON);

            // 测试 seek
            let seek_pos = Duration::from_secs(1);
            backend.seek(seek_pos).unwrap();
            assert!(approx_eq(state.lock().unwrap().current_position(), seek_pos, Duration::from_millis(50)));

            // 停止播放
            backend.stop();
            let s = state.lock().unwrap();
            assert_eq!(s.playback_state(), PlaybackState::Stopped);
            assert!(approx_eq(s.current_position(), Duration::ZERO, Duration::from_millis(50)));
            assert!(s.current_file().is_none());
        }
    }
}
