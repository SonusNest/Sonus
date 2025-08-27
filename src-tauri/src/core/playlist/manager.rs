use std::time::Duration;
use chrono::{DateTime, Utc};
use rand::prelude::SliceRandom;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::app::database;
use crate::core::library::index::Track;
use crate::core::player::audio_backend::AudioBackend;
pub(crate) use super::play_mode::PlayMode;

/**
 * Playlist Manager
 */

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub id: String,
    pub name: String,
    pub tracks: Vec<Track>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            id: String::from(""),
            name: String::from(""),
            tracks: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistManager {
    pub playlist: Playlist,
    pub current_index: Option<usize>,
    pub play_mode: PlayMode,
    original_tracks: Vec<Track>,
}

impl PlaylistManager {
    pub fn new(playlist: Playlist) -> Self {
        let original_tracks = playlist.tracks.clone();
        Self {
            playlist,
            current_index: None,
            play_mode: PlayMode::Repeat,
            original_tracks,
        }
    }

    pub fn get_playlist(&self) -> &Playlist {
        &self.playlist
    }

    pub fn get_playlist_tracks(&mut self) -> &mut Vec<Track> {
        &mut self.playlist.tracks
    }

    pub fn empty_playlist() -> Self {
        let original_tracks = vec![];
        Self {
            playlist: Playlist {
                id: String::from(""),
                name: String::from(""),
                tracks: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
            current_index: None,
            play_mode: PlayMode::Repeat,
            original_tracks,
        }
    }

    pub fn get_current_track(&self) -> Option<&Track> {
        self.current_index
            .and_then(|idx| self.playlist.tracks.get(idx))
    }

    pub fn set_current_index(&mut self, index: usize) -> Result<(), String> {
        if index < self.playlist.tracks.len() {
            self.current_index = Some(index);
            Ok(())
        } else {
            Err(format!("Index {} out of bounds", index))
        }
    }

    pub fn play(&mut self, backend: &mut AudioBackend) {
        if let Some(index) = self.current_index {
            backend.load_and_play(
                self.playlist.tracks[index].file_path.clone(),
                Duration::new(0, 0)
            ).expect("Failed to load and play track");
        } else {
            if !self.playlist.tracks.is_empty() {
                backend.load_and_play(
                    self.playlist.tracks[0].file_path.clone(),
                    Duration::new(0, 0)
                ).expect("Failed to load and play first track");
                self.current_index = Some(0);
            }
        }
    }

    pub fn pause(&self, backend: &mut AudioBackend) {
        backend.pause();
    }

    pub fn resume(&self, backend: &mut AudioBackend) {
        backend.resume();
    }

    pub fn stop(&self, backend: &mut AudioBackend) {
        backend.stop();
    }

    pub fn set_volume(&self, backend: &mut AudioBackend, v: f32) {
        backend.set_volume(v);
    }

    pub fn seek(&self, backend: &mut AudioBackend, pos: Duration) -> anyhow::Result<()> {
        backend.seek(pos)
    }

    pub fn shutdown(&self, backend: &mut AudioBackend) {
        backend.shutdown();
    }

    pub fn next_track(&mut self) -> Option<&Track> {
        match self.play_mode {
            PlayMode::Repeat => self.next_track_repeat(),
            PlayMode::Random => self.next_track_random(),
            PlayMode::Single => self.next_track_single(),
        }
        // if let Some(current) = self.current_index {
        //     if current + 1 < self.playlist.tracks.len() {
        //         self.current_index = Some(current + 1);
        //         return self.get_current_track();
        //     } else {
        //         info!("next_track: Reached end of playlist, current: {}", current); // 新增日志
        //     }
        // } else {
        //     info!("next_track: No current track selected (current_index is None)"); // 新增日志
        // }
        // None
    }

    fn next_track_repeat(&mut self) -> Option<&Track> {
        let tracks_len = self.playlist.tracks.len();
        if tracks_len == 0 {
            return None;
        }

        let next_index = match self.current_index {
            Some(current) => {
                if current + 1 < tracks_len {
                    current + 1  // 正常下一首
                } else {
                    0  // 末尾循环到开头
                }
            }
            None => 0,  // 无当前索引时从第一首开始
        };

        self.current_index = Some(next_index);
        self.get_current_track()
    }

    fn next_track_random(&mut self) -> Option<&Track> {
        let tracks_len = self.playlist.tracks.len();
        if tracks_len == 0 {
            return None;
        } else if tracks_len == 1 {
            // 只有一首时保持当前索引
            self.current_index = Some(0);
            return self.get_current_track();
        }

        // 生成不与当前索引重复的随机索引
        let current = self.current_index;
        let mut rng = rand::rng();
        let mut next_index;
        loop {
            next_index = rand::Rng::random_range(&mut rng, 0..tracks_len);
            if current != Some(next_index) {
                break;  // 确保不重复当前曲目
            }
        }

        self.current_index = Some(next_index);
        self.get_current_track()
    }

    fn next_track_single(&mut self) -> Option<&Track> {
        self.get_current_track()
    }

    pub fn previous_track(&mut self) -> Option<&Track> {
        match self.play_mode {
            PlayMode::Repeat => self.previous_track_repeat(),
            PlayMode::Single => self.get_current_track(), // 单曲模式返回当前曲目
            _ => self.previous_track_default(),
        }
    }

    fn previous_track_repeat(&mut self) -> Option<&Track> {
        let tracks_len = self.playlist.tracks.len();
        if tracks_len == 0 {
            return None;
        }

        let prev_index = match self.current_index {
            Some(current) => {
                if current == 0 {
                    tracks_len - 1  // 第一首的上一首是最后一首（循环）
                } else {
                    current - 1      // 正常上一首
                }
            }
            None => {
                tracks_len - 1  // 无当前索引时默认最后一首
            }
        };

        self.current_index = Some(prev_index);
        self.get_current_track()
    }

    fn previous_track_default(&mut self) -> Option<&Track> {
        if let Some(current) = self.current_index {
            if current > 0 {
                self.current_index = Some(current - 1);
                return self.get_current_track();
            } else {
                info!("previous_track: Reached start of playlist, current: {}", current);
            }
        } else {
            info!("previous_track: No current track selected (current_index is None)");
        }
        None
    }
    pub fn insert_at(&mut self, position: usize, track: Track) -> Result<(), String> {
        let tracks_len = self.get_playlist_tracks().len();
        if position > tracks_len {
            return Err(format!(
                "Insert position {} out of bounds (current length: {})",
                position, tracks_len
            ));
        }


        self.get_playlist_tracks().insert(position, track.clone());
        self.original_tracks.insert(position, track);

        // 调整 current_index（原逻辑保留）
        if let Some(current) = self.current_index {
            if position <= current {
                self.current_index = Some(current + 1);
            }
        } else {
            // 新增：若 current_index 为 None，且插入后列表从空变为非空，自动设为插入位置
            if tracks_len == 0 {
                self.current_index = Some(position);
            }
        }

        Ok(())
    }

    pub fn insert_at_by_index(&mut self, from_index: usize, to_position: usize) -> Result<(), String> {
        let tracks_len = self.get_playlist_tracks().len();
        // 检查源索引有效性
        if from_index >= tracks_len {
            return Err(format!(
                "Source index {} out of bounds (current length: {})",
                from_index, tracks_len
            ));
        }
        // 检查目标位置有效性（允许插入到末尾）
        if to_position > tracks_len {
            return Err(format!(
                "Target position {} out of bounds (current length: {})",
                to_position, tracks_len
            ));
        }
        // 源索引和目标位置相同无需操作
        if from_index == to_position {
            return Ok(());
        }

        // 先移除原位置音乐（注意：移除后如果目标位置在源索引之后，需要调整目标位置）
        let track = self.get_playlist_tracks().remove(from_index);
        let adjusted_position = if to_position > from_index {
            // 若目标位置在源索引之后，移除后目标位置需要减1（因为源索引元素已被移除）
            to_position - 1
        } else {
            to_position
        };
        self.get_playlist_tracks().insert(adjusted_position, track.clone());

        let original_track = self.original_tracks.remove(from_index);
        self.original_tracks.insert(adjusted_position, original_track);

        if let Some(current) = self.current_index {
            match (from_index, adjusted_position) {
                // 情况1：当前索引指向被移动的曲目 → 更新为新位置
                (f, t) if current == f => {
                    self.current_index = Some(t);
                }
                // 情况2：被移动曲目在当前索引之前 → 移除后当前索引需 -1
                (f, _) if f < current => {
                    self.current_index = Some(current - 1);
                }
                // 情况3：移动后曲目在当前索引之前 → 插入后当前索引需 +1
                (f, t) if t <= current && f > current => {
                    self.current_index = Some(current + 1);
                }
                _ => {} // 其他情况无需调整
            }
        }

        Ok(())
    }

    pub fn remove_at(&mut self, position: usize) -> Result<(), String> {
        if position >= self.get_playlist_tracks().len() {
            info!("Remove_at: Index out of bounds, position: {}", position);
            return Err(String::from("Remove_at: Index out of bounds."));
        }

        let _ = self.get_playlist_tracks().remove(position);
        let _ = self.original_tracks.remove(position);

        if let Some(current) = self.current_index {
            if position < current {
                self.current_index = Some(current - 1);
            } else if position == current {
                // 尝试切换到下一首，若失败则切换到上一首
                let new_index = if current < self.playlist.tracks.len() {
                    // 下一首存在（删除后列表长度减1，current 可能仍是有效索引）
                    current
                } else {
                    // 下一首不存在，尝试上一首
                    current.saturating_sub(1) // 避免 underflow
                };
                // 检查新索引是否有效（列表可能为空）
                if new_index < self.playlist.tracks.len() {
                    self.current_index = Some(new_index);
                } else {
                    self.current_index = None;
                }
            }

        }
        self.playlist.updated_at = Utc::now();
        Ok(())
    }

    pub fn clear(&mut self) {
        self.get_playlist_tracks().clear();
        self.original_tracks.clear();
        self.current_index = None;
        self.playlist.updated_at = Utc::now();
    }

    pub fn overwrite_playlist(&mut self, playlist: &Playlist) {
        let old_current_track = self.get_current_track().cloned(); // 保存原当前曲目
        self.playlist = playlist.clone();
        self.original_tracks = playlist.tracks.clone();

        // 若新列表中存在原当前曲目，同步索引
        self.current_index = old_current_track.as_ref()
            .and_then(|old| self.playlist.tracks.iter().position(|t| t.id == old.id));

        self.playlist.updated_at = Utc::now();
    }

    pub fn get_next_track(&self) -> Option<&Track> { // 无需 &mut self，仅查询
        let current = self.current_index?; // 直接返回 None（无当前曲目时）
        let next_index = current + 1;
        if next_index < self.playlist.tracks.len() {
            self.playlist.tracks.get(next_index)
        } else {
            info!("Get_next_track: Index out of bounds, current: {}", current);
            None
        }
    }

    pub fn get_previous_track(&self) -> Option<&Track> { // 无需 &mut self，仅查询
        let current = self.current_index?; // 直接返回 None（无当前曲目时）
        if current > 0 {
            self.playlist.tracks.get(current - 1)
        } else {
            info!("Get_previous_track: Index out of bounds, current: {}", current);
            None
        }
    }

    pub fn insert_track_to_current_next(&mut self, track: Track) -> Result<(), String> {
        tracing::info!("insert_track_to_current_next: {:?}", track.title);
        let tracks_len = self.get_playlist_tracks().len();
        // 处理 current_index 为 None 的情况：根据列表长度推断默认 current
        let current = self.current_index.unwrap_or_else(|| {
            if tracks_len == 0 {
                0 // 空列表默认 current 为 0（插入后成为第一首）
            } else {
                0 // 非空列表默认 current 为 0（插在开头后）
            }
        });

        // 计算插入位置：current + 1，但空列表时特殊处理（避免 0 + 1 = 1 越界）
        let insert_position = if tracks_len == 0 {
            0 // 空列表直接插入到 0 位置
        } else {
            // 非空列表：确保插入位置不越界（最大为 tracks_len）
            std::cmp::min(current + 1, tracks_len)
        };

        // 执行插入
        self.insert_at(insert_position, track)?;

        // 插入后更新 current_index：指向新插入的歌曲（insert_position 即新歌曲的索引）
        self.current_index = Some(insert_position);

        Ok(())
    }


    pub fn insert_track_to_current_next_by_index(&mut self, track_index: usize) -> Result<(), String> {
        let current = self.current_index.ok_or("No current track selected (current_index is None)")?; // 显式处理 None
        let tracks_len = self.get_playlist_tracks().len();

        if current >= tracks_len {
            return Err(format!(
                "Current index {} out of bounds (current length: {})",
                current, tracks_len
            ));
        }
        if track_index >= tracks_len {
            return Err(format!(
                "Track index {} out of bounds (current length: {})",
                track_index, tracks_len
            ));
        }
        if current == track_index {
            return Err("Cannot move current track to next position of itself".to_string());
        }

        let target_position = current + 1;
        self.insert_at_by_index(track_index, target_position)
    }

    pub fn insert_track_to_end(&mut self, track: Track) -> Result<(), String> {
        let end_position = self.get_playlist_tracks().len();
        let was_empty = end_position == 0; // 记录插入前是否为空列表

        // 执行插入（插入到末尾）
        self.insert_at(end_position, track)?;

        // 处理 current_index 为空的情况：空列表插入后自动选中第一首（索引 0）
        if was_empty && self.current_index.is_none() {
            self.current_index = Some(0);
        }

        Ok(())
    }

    pub fn insert_track_to_end_by_index(&mut self, track_index: usize) -> Result<(), String> {
        let tracks_len = self.get_playlist_tracks().len();
        // 检查待移动音乐索引有效性
        if track_index >= tracks_len {
            return Err(format!(
                "Track index {} out of bounds (current length: {})",
                track_index, tracks_len
            ));
        }
        // 已经在末尾则无需操作
        if track_index == tracks_len - 1 {
            return Ok(());
        }

        // 目标位置是当前长度（移除后长度会减1，insert 到新长度即可）
        let target_position = tracks_len;
        self.insert_at_by_index(track_index, target_position)
    }

    pub fn set_play_mode(&mut self, new_mode: PlayMode) {
        let play_mode = self.play_mode.clone();

        if play_mode == new_mode {
            return;  // 模式不变，无需操作
        }

        // 根据模式切换处理曲目列表
        match (play_mode, new_mode) {
            (_, PlayMode::Random) => {
                // 切换到随机模式：打乱原始列表并更新当前列表
                self.switch_to_random_mode();
            }
            (PlayMode::Random, PlayMode::Repeat) => {
                // 从随机切换到列表循环：恢复原始顺序
                self.switch_to_repeat_mode();
            }
            _ => {}  // 其他模式切换（如单曲循环↔列表循环）无需调整曲目顺序
        }

        self.play_mode = new_mode;
    }

    fn switch_to_random_mode(&mut self) {
        let tracks_len = self.original_tracks.len();
        if tracks_len == 0 {
            self.playlist.tracks.clear();
            self.current_index = None; // 明确置空
            return;
        }
        if tracks_len <= 1 {
            // 曲目数量≤1时无需打乱
            self.playlist.tracks = self.original_tracks.clone();
            return;
        }

        // 打乱原始列表生成随机顺序
        let mut shuffled = self.original_tracks.clone();
        let mut rng = rand::rng();  // 随机数生成器
        shuffled.shuffle(&mut rng);  // 打乱列表

        // 确保当前曲目在打乱后列表中，更新索引
        self.sync_current_index_after_shuffle(&shuffled);

        self.playlist.tracks = shuffled;
    }

    fn switch_to_repeat_mode(&mut self) {
        let original = self.original_tracks.clone();
        // 找到当前曲目在原始列表中的位置
        self.sync_current_index_after_restore(&original);
        self.playlist.tracks = original;
    }

    // 打乱后同步当前索引（确保当前曲目位置正确）
    fn sync_current_index_after_shuffle(&mut self, shuffled: &[Track]) {
        if let Some(current) = self.current_index {
            // 获取当前播放的曲目（来自原始列表）
            if let Some(current_track) = self.original_tracks.get(current) {
                // 在打乱列表中查找该曲目的新位置
                if let Some(new_index) = shuffled.iter().position(|t| t.id == current_track.id) {
                    self.current_index = Some(new_index);
                    return;
                }
            }
        }
        // 无法找到曲目时重置索引
        self.current_index = if shuffled.is_empty() { None } else { Some(0) };
    }

    fn sync_current_index_after_restore(&mut self, original: &[Track]) {
        if let Some(current) = self.current_index {
            // 获取当前播放的曲目（来自随机列表）
            if let Some(current_track) = self.playlist.tracks.get(current) {
                // 在原始列表中查找该曲目的位置
                if let Some(original_index) = original.iter().position(|t| t.id == current_track.id) {
                    self.current_index = Some(original_index);
                    return;
                }
            }
        }
        // 无法找到曲目时重置索引
        self.current_index = if original.is_empty() { None } else { Some(0) };
    }
}
