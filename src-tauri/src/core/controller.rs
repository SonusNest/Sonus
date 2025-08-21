use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use super::library::index::Track;
use super::player::audio_backend::AudioBackend;
use super::player::state::{PlaybackState, SharedState};
use super::playlist::manager::{Playlist, PlaylistManager};


pub type SharedPlayerController = Arc<Mutex<PlayerController>>;

pub fn new_shared_player_controller(state: SharedState) -> anyhow::Result<SharedPlayerController> {
    let playlist = Playlist::new();
    let playlist_manager = PlaylistManager::new(playlist);
    let backend = AudioBackend::new(state.clone())?;
    Ok(Arc::new(Mutex::new(PlayerController {
        playlist_manager,
        backend,
        state
    })))
}
pub struct PlayerController {
    playlist_manager: PlaylistManager,
    backend: AudioBackend,
    state: SharedState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayMode {
    Single,
    Queue
}

impl PlayerController {
    pub fn new(state: SharedState) -> anyhow::Result<Self> {
        let playlist = Playlist::new();
        let playlist_manager = PlaylistManager::new(playlist);
        let backend = AudioBackend::new(state.clone())?;
        Ok(Self { playlist_manager, backend, state })
    }

    pub fn play_to_playlist(&mut self, tracks: Vec<Track>, play_mode: PlayMode) -> anyhow::Result<()> {
        match play_mode {
            PlayMode::Single => {
                tracing::info!("play_to_playlist: {:?}", tracks[0].title);
                self.playlist_manager.insert_track_to_current_next(tracks[0].clone()).expect("Failed to insert track to current next");
                tracing::info!("into next track: {:?}", tracks[0].title);
                tracing::info!("current track: {:?}", self.playlist_manager.get_current_track());
                self.playlist_manager.next_track();
                tracing::info!("current track: {:?}", self.playlist_manager.get_current_track());
                self.play();
                tracing::info!("playlist: {:?}", self.playlist_manager.get_playlist());
                tracing::info!("playlist tracks: {:?}", self.playlist_manager.get_playlist_tracks());
            }
            PlayMode::Queue => {
                let mut playlist = Playlist::new();
                playlist.tracks = tracks;
                self.playlist_manager.overwrite_playlist(&playlist);
                self.playlist_manager.next_track();
            }
        }
        Ok(())
    }

    pub fn play_from<P: AsRef<Path>>(&mut self, path: P, position: Duration) -> anyhow::Result<()> {
        self.backend.load_and_play(path, position)
    }
    pub fn play(&mut self)  {
        self.playlist_manager.play(&mut self.backend);
    }

    pub fn pause(&mut self) {
        self.playlist_manager.pause(&mut self.backend);
    }

    pub fn resume(&mut self) {
        self.playlist_manager.resume(&mut self.backend);
    }

    pub fn stop(&mut self) {
        self.playlist_manager.stop(&mut self.backend);
    }

    pub fn set_volume(&mut self, v: f32) {
        self.playlist_manager.set_volume(&mut self.backend, v);
    }

    pub fn seek(&mut self, pos: Duration) -> anyhow::Result<()> {
        self.playlist_manager.seek(&mut self.backend, pos)
    }

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

    pub fn shutdown(&mut self) {
        self.playlist_manager.shutdown(&mut self.backend);
    }
}
