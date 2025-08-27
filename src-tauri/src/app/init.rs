/**
 * Init.rs
 *
 * @description
 * Initialize the application.
 * Here is the code that initializes when the program first runs, initializes the database and configures the application.
 */
use rusqlite::{Connection, Result};

use super::database::{
    Config,
    connection,
    get_config_value
};
use super::window;


fn init_db() -> Result<Connection> {
    #[cfg(windows)]
    {
        let conn = connection();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS config (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT
        )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS music (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT,
            album TEXT,
            artist TEXT,
            album_artist TEXT,
            composer TEXT,
            lyricist TEXT,
            genre TEXT,
            release_date TIMESTAMP,
            track_number INTEGER,
            disc_number INTEGER,
            disc_total INTEGER,
            bpm INTEGER,
            duration INTEGER,
            cover_art TEXT,
            audio_format TEXT,
            audio_size INTEGER,
            bitrate INTEGER,
            sample_rate INTEGER,
            path_type TEXT,
            file_path INTEGER,
            create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            copyright TEXT,
            remark TEXT,
            is_love INTEGER,
            lyrics TEXT,
            hash TEXT
        )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlist (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT,
            create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            remark TEXT
        )",
            (),
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS playlist_music (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            playlist_id INTEGER,
            music_id INTEGER,
            create_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            update_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            remark TEXT
        )",
            (),
        )?;

        init_config(&conn)?;

        Ok(conn)
    }
}

fn init_config(conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("material", "2"),
    )?; // Window Material
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("auto_start", "0"),
    )?; // Autio Start
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("close_action", "0"),
    )?; // Close Button Action
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("start_play", "0"),
    )?; // App start auto play
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("memory_play", "0"),
    )?; // Memory lat play location
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("fade_in_out", "0"),
    )?; // Fade In Out
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("loudness_balance", "0"),
    )?; // Loudness Balance
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("audio_enhancement", "0"),
    )?; // Audio Enhancement
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("dynamic_backdrop", "0"),
    )?; // Dynamic Backdrop

    // Shortcut Keys
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("global_shortcut", "0"),
    )?; // Global Shortcut
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("system_shortcut", "0"),
    )?; // System Shortcut
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("play_or_pause", "0"),
    )?; // Play or Pause
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("next_track", "0"),
    )?; // Next Track
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("previous_track", "0"),
    )?; // Previous Track
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("mute", "0"),
    )?; // Mute
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("volume_up", "0"),
    )?; // Volume Up
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("volume_down", "0"),
    )?; // Volume Down
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("mini_mode", "0"),
    )?; // Mini Mode
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("mini_mode_hide", "0"),
    )?; // Mini Mode Hide
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("immersion_mode", "0"),
    )?; // Immersion Mode
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("love_song", "0"),
    )?; // Love Song
    conn.execute(
        "INSERT OR IGNORE INTO config (key, value) VALUES (?, ?)",
        ("desktop_lyrics", "0"),
    )?; // Desktop Lyrics

    Ok(())
}

fn window_init(app: &tauri::AppHandle) {
    #[cfg(windows)]
    {
        let materiel: Config =
            get_config_value(&connection(), "material").unwrap();
        window::set_window_material(&app.clone(), materiel.value.parse().unwrap());
    }
}

pub fn init(app: &tauri::AppHandle) {
    init_db().unwrap();
    window_init(&app);
}
