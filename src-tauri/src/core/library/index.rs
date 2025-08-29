use chrono::{DateTime, NaiveDate, NaiveDateTime, ParseError, TimeZone, Utc};
use rusqlite::{Row};
use rusqlite::types::{Type, ValueRef};
use serde::{Deserialize, Serialize};
use tracing::warn;
use crate::app::database::{connection, query_with_params};

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

    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        // 辅助函数：将逗号分隔的字符串转换为 Vec<String>
        fn str_to_vec(s: Option<String>) -> Option<Vec<String>> {
            s.map(|s| {
                s.split(',')
                    .map(|item| item.trim().to_string())
                    .filter(|item| !item.is_empty())
                    .collect()
            })
        }

        // 辅助函数：将字符串解析为 DateTime<Utc>
        fn str_to_datetime(col_idx: i32, row: &Row, col: usize) -> Option<DateTime<Utc>> {
            // 先获取原始值，判断字段类型
            match row.get_ref_unwrap(col) {
                // 处理null值
                ValueRef::Null => return None,

                // 处理整数类型（可能是Unix时间戳，秒或毫秒）
                ValueRef::Integer(timestamp) => {
                    // 尝试解析为秒级时间戳（通常是10位数字）
                    if let Ok(secs) = i64::try_from(timestamp) {
                        // 区分秒级（10位）和毫秒级（13位）时间戳
                        let secs = if secs > 1_000_000_000_000 {
                            secs / 1000 // 毫秒转秒
                        } else {
                            secs
                        };
                        return Utc.timestamp_opt(secs, 0).single();
                    }
                }

                // 处理字符串类型
                ValueRef::Text(s) => {
                    let s = match std::str::from_utf8(s) {
                        Ok(s) => s.trim(),
                        Err(_) => {
                            warn!("Column {} contains non-UTF8 string", col_idx);
                            return None;
                        }
                    };

                    if s.is_empty() {
                        return None;
                    }

                    // 先尝试解析为纯日期
                    let date_formats = ["%Y-%m-%d", "%Y-%m", "%Y"];
                    for fmt in &date_formats {
                        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
                            return Some(Utc.from_utc_datetime(&date.and_hms(0, 0, 0)));
                        }
                    }

                    // 再尝试解析为日期时间
                    let datetime_formats = [
                        "%Y-%m-%dT%H:%M:%SZ",
                        "%Y-%m-%dT%H:%M:%S",
                        "%Y-%m-%d %H:%M:%S",
                    ];
                    for fmt in &datetime_formats {
                        if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
                            return Some(Utc.from_utc_datetime(&dt));
                        }
                    }
                }

                // 其他类型（如Blob）视为无效
                _ => {
                    warn!("Column {} contains unsupported type", col_idx);
                    return None;
                }
            }

            // 所有解析都失败
            warn!("Unable to parse column {} value: {}", col_idx, col);
            None
        }

        Ok(Self {
            id: row.get(0)?,
            title: row.get(1)?,
            album: row.get(2)?,
            artist: str_to_vec(row.get(3)?),
            album_artist: row.get(4)?,
            composer: str_to_vec(row.get(5)?),
            lyricist: str_to_vec(row.get(6)?),
            genre: str_to_vec(row.get(7)?),
            release_date: str_to_datetime(8,row,8),
            track_number: row.get(9)?,
            disc_number: row.get(10)?,
            bpm: row.get(11)?,
            duration: row.get(12)?,
            cover_art: str_to_vec(row.get(13)?),
            audio_format: row.get(14)?,
            audio_size: row.get(15)?,
            bitrate: row.get(16)?,
            sample_rate: row.get(17)?,
            file_path: row.get(18)?,
            create_time: str_to_datetime(19,row, 19),
            update_time: str_to_datetime(20,row, 20),
            copyright: row.get(21)?,
            remark: row.get(22)?,
            path_type: row.get(23)?,
            is_love: row.get(24)?,
            hash: row.get(25)?,
            disc_total: row.get(26)?,
            lyrics: row.get(27)?,
        })
    }
}

pub fn get_all_songs(limit: usize, offset: usize) -> rusqlite::Result<Vec<Track>> {
    use crate::app::database::connection;

    let conn = connection();
    query_with_params(
        &conn,
        r#"SELECT
            id, title, album, artist, album_artist, composer, lyricist, genre,
            release_date, track_number, disc_number, bpm, duration, cover_art,
            audio_format, audio_size, bitrate, sample_rate, file_path, create_time,
            update_time, copyright, remark, path_type, is_love, hash, disc_total, lyrics
           FROM music
           LIMIT ? OFFSET ?"#,
        &[&limit, &offset],
        Track::from_row
    )
}