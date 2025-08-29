use std::path::{Path, PathBuf};
use std::sync::MutexGuard;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::core::controller::{PlayMode, PlayerController, SharedPlayerController};
use crate::core::library::index::Track;
use crate::core::player::state::{PlaybackState};
use crate::core::playlist::manager::Playlist;
use crate::core::playlist::play_mode;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct DurationMs(pub u64);

impl From<Duration> for DurationMs {
    fn from(dur: Duration) -> Self {
        DurationMs(dur.as_millis() as u64)
    }
}

impl From<DurationMs> for Duration {
    fn from(ms: DurationMs) -> Self {
        Duration::from_millis(ms.0)
    }
}

// 播放状态的字符串表示（方便前端处理）
#[derive(Debug, Serialize, Deserialize)]
pub enum PlaybackStateStr {
    Stopped,
    Playing,
    Paused,
}

impl From<PlaybackState> for PlaybackStateStr {
    fn from(state: PlaybackState) -> Self {
        match state {
            PlaybackState::Stopped => PlaybackStateStr::Stopped,
            PlaybackState::Playing => PlaybackStateStr::Playing,
            PlaybackState::Paused => PlaybackStateStr::Paused,
        }
    }
}

// 播放器状态快照（前端友好格式）
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerStatus {
    pub state: PlaybackStateStr,
    pub volume: f32,
    pub current_position: DurationMs,
    pub total_duration: Option<DurationMs>,
    pub current_file: Option<String>,
    pub current_track: Option<Track>,
    pub current_playlist: Playlist,
    pub play_mode: play_mode::PlayMode,
}

fn get_controller_lock(controller: &SharedPlayerController) -> MutexGuard<PlayerController> {
    controller
        .lock()
        .expect("Failed to lock player controller (is it poisoned?)")
}

#[tauri::command]
pub fn play_to_playlist(
    controller: State<SharedPlayerController>,
    tracks: Vec<Track>,
    play_mode: PlayMode,
) -> Result<(), String> {
    tracing::info!("======= play_to_playlist is called =======");
    tracing::info!("play_to_playlist: {:?}", tracks.len());
    tracing::info!("play_mode: {:?}", play_mode);
    if let Some(first_track) = tracks.first() {
        tracing::info!("  track path: {:?}", first_track.file_path);
    }
    let mut controller = get_controller_lock(&controller); // 获取可变锁
    controller
        .play_to_playlist(tracks, play_mode)
        .map_err(|e| {
            tracing::error!("play_to_playlist error: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub fn play_from(
    controller: State<SharedPlayerController>,
    path: PathBuf,
    position: DurationMs,
) -> Result<(), String> {
    let mut controller = get_controller_lock(&controller);
    controller
        .play_from(path, position.into())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn play(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.play();
}

#[tauri::command]
pub fn pause(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.pause();
}

#[tauri::command]
pub fn resume(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.resume();
}

#[tauri::command]
pub fn stop(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.stop();
}

#[tauri::command]
pub fn set_volume(controller: State<SharedPlayerController>, volume: f32) {
    let mut controller = get_controller_lock(&controller);
    controller.set_volume(volume);
}

#[tauri::command]
pub fn seek(
    controller: State<SharedPlayerController>,
    position: DurationMs,
) -> Result<(), String> {
    let mut controller = get_controller_lock(&controller);
    controller
        .seek(position.into())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_player_status(controller: State<SharedPlayerController>) -> PlayerStatus {
    let controller = get_controller_lock(&controller); 
    let (state, volume, pos, total, file) = controller.snapshot();

    PlayerStatus {
        state: state.into(),
        volume,
        current_position: pos.into(),
        total_duration: total.map(|d| d.into()),
        current_file: file,
        current_track: controller.playlist_manager.get_current_track().cloned(),
        current_playlist: controller.playlist_manager.get_playlist().clone(),
        play_mode: controller.playlist_manager.play_mode.clone(),
    }
}

#[tauri::command]
pub fn shutdown_player(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.shutdown();
}

#[tauri::command]
pub fn next_track(controller: State<SharedPlayerController>) -> Result<Option<Track>, String> {
    tracing::info!("next_track called");
    let mut controller = get_controller_lock(&controller);

    let next = controller.playlist_manager.next_track().cloned();

    if next.is_some() {
        controller.play();
    }

    Ok(next)
}

#[tauri::command]
pub fn previous_track(controller: State<SharedPlayerController>) -> Result<Option<Track>, String> {
    tracing::info!("previous_track called");
    let mut controller = get_controller_lock(&controller);

    let prev = controller.playlist_manager.previous_track().cloned();

    if prev.is_some() {
        controller.play();
    }

    Ok(prev)
}


#[tauri::command]
pub fn insert_track_at(
    controller: State<SharedPlayerController>,
    position: usize,
    track: Track
) -> Result<Playlist, String> {
    tracing::info!("insert_track_at called: position {}", position);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.insert_at(position, track)?;
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn insert_track_after_current(
    controller: State<SharedPlayerController>,
    track: Track
) -> Result<Playlist, String> {
    tracing::info!("insert_track_after_current called");
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.insert_track_to_current_next(track)?;
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn add_track_to_end(
    controller: State<SharedPlayerController>,
    track: Track
) -> Result<Playlist, String> {
    tracing::info!("add_track_to_end called");
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.insert_track_to_end(track)?;
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn move_track(
    controller: State<SharedPlayerController>,
    from_index: usize,
    to_position: usize
) -> Result<Playlist, String> {
    tracing::info!("move_track called: from {} to {}", from_index, to_position);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.insert_at_by_index(from_index, to_position)?;
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn remove_track(
    controller: State<SharedPlayerController>,
    position: usize
) -> Result<Playlist, String> {
    tracing::info!("remove_track called: position {}", position);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.remove_at(position)?;
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn clear_playlist(
    controller: State<SharedPlayerController>
) -> Result<Playlist, String> {
    tracing::info!("clear_playlist called");
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.clear();
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn overwrite_playlist(
    controller: State<SharedPlayerController>,
    playlist: Playlist
) -> Result<Playlist, String> {
    tracing::info!("overwrite_playlist called: {}", playlist.name);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.overwrite_playlist(&playlist);
    Ok(controller.playlist_manager.get_playlist().clone())
}

#[tauri::command]
pub fn set_play_mode(
    controller: State<SharedPlayerController>,
    mode: play_mode::PlayMode
) -> Result<play_mode::PlayMode, String> {
    tracing::info!("set_play_mode called: {:?}", mode);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.set_play_mode(mode.clone());
    Ok(controller.playlist_manager.play_mode.clone())
}

pub fn get_play_mode(
    controller: State<SharedPlayerController>
) -> Result<play_mode::PlayMode, String> {
    let controller = get_controller_lock(&controller);
    Ok(controller.playlist_manager.play_mode.clone())
}

#[tauri::command]
pub fn get_current_index(
    controller: State<SharedPlayerController>
) -> Result<Option<usize>, String> {
    let controller = get_controller_lock(&controller);
    Ok(controller.playlist_manager.current_index)
}

#[tauri::command]
pub fn set_and_play_index(
    controller: State<SharedPlayerController>,
    index: usize
) -> Result<Option<Track>, String> {
    tracing::info!("set_and_play_index called: {}", index);
    let mut controller = get_controller_lock(&controller);
    controller.playlist_manager.set_current_index(index)?;
    controller.play();
    Ok(controller.playlist_manager.get_current_track().cloned())
}