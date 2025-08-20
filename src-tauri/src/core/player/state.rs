use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug)]
pub struct PlayerState {
    playback_state: PlaybackState,
    volume: f32,
    current_position: Duration,
    total_duration: Option<Duration>,
    current_file: Option<String>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            volume: 1.0,
            current_position: Duration::from_secs(0),
            total_duration: None,
            current_file: None,
        }
    }
}

impl PlayerState {
    pub fn new() -> Self { Self::default() }

    pub fn playback_state(&self) -> PlaybackState { self.playback_state }
    pub fn set_playback_state(&mut self, state: PlaybackState) { self.playback_state = state; }

    pub fn volume(&self) -> f32 { self.volume }
    pub fn set_volume(&mut self, volume: f32) { self.volume = volume.clamp(0.0, 1.0); }

    pub fn current_position(&self) -> Duration { self.current_position }
    pub fn set_current_position(&mut self, position: Duration) { self.current_position = position; }

    pub fn total_duration(&self) -> Option<Duration> { self.total_duration }
    pub fn set_total_duration(&mut self, duration: Option<Duration>) { self.total_duration = duration; }

    pub fn current_file(&self) -> Option<&str> { self.current_file.as_deref() }
    pub fn set_current_file(&mut self, file_path: Option<String>) { self.current_file = file_path; }

    // 便捷方法
    pub fn is_playing(&self) -> bool { self.playback_state == PlaybackState::Playing }
    pub fn is_paused(&self) -> bool { self.playback_state == PlaybackState::Paused }
    pub fn is_stopped(&self) -> bool { self.playback_state == PlaybackState::Stopped }
}

pub type SharedState = Arc<Mutex<PlayerState>>;
pub fn new_shared_state() -> SharedState { Arc::new(Mutex::new(PlayerState::new())) }
