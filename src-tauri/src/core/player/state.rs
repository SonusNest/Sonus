use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use crate::core::library::index::Track;
use crate::core::playlist::manager::Playlist;
use crate::core::playlist::play_mode::PlayMode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerState {
    playback_state: PlaybackState,
    volume: f32,
    current_position: Duration,
    total_duration: Option<Duration>,
    current_file: Option<String>,
    current_track: Option<Track>,
    current_playlist: Playlist,
    current_play_mode: PlayMode,
    current_index: Option<usize>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            volume: 1.0,
            current_position: Duration::from_secs(0),
            total_duration: None,
            current_file: None,
            current_track: None,
            current_playlist: Playlist::new(),
            current_play_mode: PlayMode::Single,
            current_index: None,
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

    pub fn current_track(&self) -> Option<Track> { self.current_track.clone() }
    pub fn set_current_track(&mut self, track: Option<Track>) { self.current_track = track; }

    pub fn current_playlist(&self) -> Playlist { self.current_playlist.clone() }
    pub fn set_current_playlist(&mut self, playlist: Playlist) { self.current_playlist = playlist; }

    pub fn current_play_mode(&self) -> PlayMode { self.current_play_mode }
    pub fn set_current_play_mode(&mut self, play_mode: PlayMode) { self.current_play_mode = play_mode; }
    
    pub fn current_index(&self) -> Option<usize> { self.current_index }
    pub fn set_current_index(&mut self, index: Option<usize>) { self.current_index = index; }

    // 便捷方法
    pub fn is_playing(&self) -> bool { self.playback_state == PlaybackState::Playing }
    pub fn is_paused(&self) -> bool { self.playback_state == PlaybackState::Paused }
    pub fn is_stopped(&self) -> bool { self.playback_state == PlaybackState::Stopped }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DurationWrapper(pub Duration);

impl From<Duration> for DurationWrapper {
    fn from(dur: Duration) -> Self {
        DurationWrapper(dur)
    }
}

impl From<DurationWrapper> for Duration {
    fn from(wrapper: DurationWrapper) -> Self {
        wrapper.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StateSnapshot {
    pub playback_state: PlaybackState,
    pub volume: f32,
    pub current_position: u64, // 以毫秒为单位
    pub total_duration: Option<u64>, // 以毫秒为单位
    pub current_file: Option<String>,
    pub current_track: Option<Track>,
    pub current_playlist: Playlist,
    pub current_play_mode: PlayMode,
    pub current_index: Option<usize>,
}

impl From<&PlayerState> for StateSnapshot {
    fn from(state: &PlayerState) -> Self {
        StateSnapshot {
            playback_state: state.playback_state(),
            volume: state.volume(),
            current_position: state.current_position().as_millis() as u64,
            total_duration: state.total_duration().map(|d| d.as_millis() as u64),
            current_file: state.current_file().map(|s| s.to_string()),
            current_track: state.current_track().map(|t| t.into()),
            current_playlist: state.current_playlist().clone(),
            current_play_mode: state.current_play_mode().into(),
            current_index: state.current_index(),
        }
    }
}

pub type SharedState = Arc<Mutex<PlayerState>>;
pub fn new_shared_state() -> SharedState { Arc::new(Mutex::new(PlayerState::new())) }
