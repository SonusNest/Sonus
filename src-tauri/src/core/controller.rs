use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use serde::{Deserialize, Serialize};
use super::library::index::Track;
use super::player::audio_backend::AudioBackend;
use super::player::state::{PlaybackState, SharedState};
use super::playlist::manager::{Playlist, PlaylistManager};
use tauri::AppHandle;
use crate::core::playlist::play_mode;

pub type SharedPlayerController = Arc<Mutex<PlayerController>>;

pub fn new_shared_player_controller(state: SharedState, app_handle: AppHandle) -> anyhow::Result<SharedPlayerController> {
    let playlist = Playlist::new();
    let playlist_manager = PlaylistManager::new(playlist);
    let backend = AudioBackend::new(state.clone(), app_handle)?;
    Ok(Arc::new(Mutex::new(PlayerController {
        playlist_manager,
        backend,
        state
    })))
}
pub struct PlayerController {
    pub(crate) playlist_manager: PlaylistManager,
    backend: AudioBackend,
    state: SharedState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlayMode {
    Single,
    Queue
}

impl PlayerController {
    pub fn new(state: SharedState, app_handle: AppHandle) -> anyhow::Result<Self> {
        let playlist = Playlist::new();
        let playlist_manager = PlaylistManager::new(playlist);
        let backend = AudioBackend::new(state.clone(), app_handle)?;
        Ok(Self { playlist_manager, backend, state })
    }

    pub fn play_to_playlist(&mut self, tracks: Vec<Track>, play_mode: PlayMode) -> anyhow::Result<()> {
        match play_mode {
            PlayMode::Single => {
                // 原逻辑：插入单首到下一首并播放
                self.playlist_manager.insert_track_to_current_next(tracks[0].clone())
                    .expect("Failed to insert track to current next");
                // self.playlist_manager.next_track();
                self.play();
                // 新增：同步播放列表到状态
                self.sync_all_to_state();
            }
            PlayMode::Queue => {
                // 原逻辑：覆盖播放列表
                let mut playlist = Playlist::new();
                playlist.tracks = tracks;
                self.playlist_manager.overwrite_playlist(&playlist);
                self.playlist_manager.next_track();
                // 新增：同步播放列表和模式到状态
                self.sync_all_to_state();
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
    
    pub fn next_track(&mut self) {
        self.playlist_manager.next_track();
    }
    
    pub fn previous_track(&mut self) {
        self.playlist_manager.previous_track();
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

    pub fn overwrite_playlist(&mut self, playlist: &Playlist) {
        // 调用PlaylistManager的方法修改播放列表
        self.playlist_manager.overwrite_playlist(playlist);
        // 同步到SharedState
        self.sync_all_to_state();
    }

    pub fn set_play_mode(&mut self, new_mode: play_mode::PlayMode) {
        // 调用PlaylistManager的方法修改播放模式（可能会改变播放列表顺序）
        self.playlist_manager.set_play_mode(new_mode);
        // 同步播放模式和播放列表到SharedState（因为切换模式可能打乱列表）
        self.sync_all_to_state();
    }

    pub fn insert_track_at(&mut self, position: usize, track: Track) -> Result<(), String> {
        let result = self.playlist_manager.insert_at(position, track);
        // 无论插入成功与否，只要可能修改了列表，就同步状态
        if result.is_ok() {
            self.sync_all_to_state();
        }
        result
    }

    pub fn remove_track_at(&mut self, position: usize) -> Result<(), String> {
        let result = self.playlist_manager.remove_at(position);
        if result.is_ok() {
            self.sync_all_to_state();
        }
        result
    }

    pub fn clear_playlist(&mut self) {
        self.playlist_manager.clear();
        self.sync_all_to_state();
    }


    // 辅助方法：同步所有相关数据到SharedState
    fn sync_all_to_state(&mut self) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        state.set_current_playlist(self.playlist_manager.get_playlist().clone());
        state.set_current_track(self.playlist_manager.get_current_track().cloned());
        state.set_current_play_mode(self.playlist_manager.play_mode);
        state.set_current_index(self.playlist_manager.current_index);
    }
}
