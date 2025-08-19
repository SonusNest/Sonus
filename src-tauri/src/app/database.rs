use serde::Serialize;
use rusqlite::{Connection, Result};

#[derive(Debug, Serialize)]
pub struct Config {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct Music {
    pub id: i64,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub album_artist: String,
    pub composer: String,
    pub lyricist: String,
    pub genre: String,
    pub release_date: String,
    pub track_number: i64,
    pub disc_number: i64,
    pub bpm: i64,
    pub duration: i64,
    pub cover_art: String,
    pub audio_format: String,
    pub audio_size: i64,
    pub bitrate: i64,
    pub sample_rate: i64,
    pub file_path: String,
    pub create_time: String,
    pub update_time: String,
    pub copyright: String,
    pub remark: String,
}

#[derive(Debug, Serialize)]
pub struct Playlist {
    pub id: i64,
    pub name: String,
    pub create_time: String,
    pub update_time: String,
    pub remark: String,
}

#[derive(Debug, Serialize)]
pub struct PlaylistMusic {
    pub id: i64,
    pub playlist_id: i64,
    pub music_id: i64,
    pub create_time: String,
    pub update_time: String,
    pub remark: String,
}

pub fn connection() -> Connection {
    #[cfg(windows)]
    {
        let app_data_dir = std::env::var("APPDATA").unwrap();
        let sonus_dir = &format!("{}\\Sonus", app_data_dir);
        std::fs::create_dir_all(sonus_dir).unwrap();
        let conn = Connection::open(format!("{}\\sonus.db", sonus_dir)).unwrap();
        conn
    }
}

pub fn get_config_value(conn: &Connection, key: &str) -> Result<Config> {
    let mut stmt = conn.prepare("SELECT key, value FROM config WHERE key = ?")?;
    let config = stmt.query_row([key], |row| {
        Ok(Config {
            key: row.get(0)?,
            value: row.get(1)?,
        })
    })?;

    Ok(config)
}

pub fn set_config_value(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute("UPDATE config SET value = ? WHERE key = ?", [value, key])?;
    Ok(())
}

pub fn execute(conn: &Connection, cmd: &str) -> Result<usize> {
    Ok(conn.execute(cmd, [])?)
}

pub fn execute_with_params(conn: &Connection, cmd: &str, params: &[&dyn rusqlite::types::ToSql]) -> Result<usize> {
    Ok(conn.execute(cmd, params)?)
}

