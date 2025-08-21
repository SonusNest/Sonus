use serde::Serialize;
use rusqlite::{Connection, Result, Row, ToSql};

#[derive(Debug, Serialize)]
pub struct Config {
    pub key: String,
    pub value: String,
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

pub fn query_with_params<T, F>(
    conn: &Connection,
    sql: &str,
    params: &[&dyn ToSql],
    mapper: F
) -> Result<Vec<T>>
where
    F: FnMut(&Row) -> Result<T>
{
    let mut stmt = conn.prepare(sql)?;
    let rows = stmt.query_map(params, mapper)?;
    rows.collect()
}

pub fn execute(conn: &Connection, cmd: &str) -> Result<usize> {
    Ok(conn.execute(cmd, [])?)
}

pub fn execute_with_params(conn: &Connection, cmd: &str, params: &[&dyn ToSql]) -> Result<usize> {
    Ok(conn.execute(cmd, params)?)
}

