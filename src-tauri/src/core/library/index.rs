use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: Option<usize>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub artist: Option<Vec<String>>,
    pub album_artist: Option<String>,
    pub composer: Option<Vec<String>>,
    pub lyricist: Option<Vec<String>>,
    pub genre: Option<Vec<String>>,
    pub release_date: Option<DateTime<Utc>>,
    pub track_number: Option<u16>,
    pub disc_number: Option<u16>,
    pub bpm: Option<u16>,
    pub duration: u32,
    pub cover_art: Option<Vec<String>>,
    pub audio_format: Option<String>,
    pub audio_size: u64,
    pub bitrate: Option<u32>,
    pub sample_rate: Option<u32>,
    pub file_path: String,
    pub create_time: Option<DateTime<Utc>>,
    pub update_time: Option<DateTime<Utc>>,
    pub copyright: Option<String>,
    pub remark: Option<String>,
    pub path_type: u8,
    pub is_love: u8,
    pub hash: String,
    pub disc_total: Option<u16>,
    pub lyrics: Option<String>,
}

impl Track {
    pub fn new() -> Self {
        Self {
            id: None,
            title: None,
            album: None,
            artist: None,
            album_artist: None,
            composer: None,
            lyricist: None,
            genre: None,
            release_date: None,
            track_number: None,
            disc_number: None,
            bpm: None,
            duration: 0,
            cover_art: None,
            audio_format: None,
            audio_size: 0,
            bitrate: None,
            sample_rate: None,
            file_path: String::new(),
            create_time: None,
            update_time: None,
            copyright: None,
            remark: None,
            path_type: 0,
            is_love: 0,
            hash: String::new(),
            disc_total: None,
            lyrics: None,
        }
    }
}
