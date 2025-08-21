use std::path::{Path, PathBuf};
use std::sync::MutexGuard;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tauri::State;
use crate::core::controller;
use crate::core::controller::{PlayMode, PlayerController, SharedPlayerController};
use crate::core::library::index::Track;
use crate::core::player::state::{PlaybackState, StateSnapshot};

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
}

// 辅助函数：获取控制器的可变锁（简化代码）
fn get_controller_lock(controller: &SharedPlayerController) -> MutexGuard<PlayerController> {
    controller
        .lock()
        .expect("Failed to lock player controller (is it poisoned?)")
}

// 1. 播放指定曲目列表
#[tauri::command]
pub fn play_to_playlist(
    controller: State<SharedPlayerController>, // 使用新的共享类型
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

// 2. 从指定位置播放单个文件
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

// 3. 继续播放
#[tauri::command]
pub fn play(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.play();
}

// 4. 暂停播放
#[tauri::command]
pub fn pause(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.pause();
}

// 5. 恢复播放
#[tauri::command]
pub fn resume(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.resume();
}

// 6. 停止播放
#[tauri::command]
pub fn stop(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.stop();
}

// 7. 设置音量（0.0-1.0范围）
#[tauri::command]
pub fn set_volume(controller: State<SharedPlayerController>, volume: f32) {
    let mut controller = get_controller_lock(&controller);
    controller.set_volume(volume);
}

// 8. 调整播放位置
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

// 9. 获取当前播放器状态
#[tauri::command]
pub fn get_player_status(controller: State<SharedPlayerController>) -> PlayerStatus {
    let controller = get_controller_lock(&controller); // 这里不需要可变锁，但统一用同一个函数
    let (state, volume, pos, total, file) = controller.snapshot();

    PlayerStatus {
        state: state.into(),
        volume,
        current_position: pos.into(),
        total_duration: total.map(|d| d.into()),
        current_file: file,
    }
}

// 10. 关闭播放器（释放资源）
#[tauri::command]
pub fn shutdown_player(controller: State<SharedPlayerController>) {
    let mut controller = get_controller_lock(&controller);
    controller.shutdown();
}

