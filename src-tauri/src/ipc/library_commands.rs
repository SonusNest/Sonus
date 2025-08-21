use crate::core::library;
use crate::core::library::index::Track;

#[tauri::command]
pub async fn get_all_songs(limit: usize, offset: usize) -> Result<Vec<Track>, String> {
    let tracks = library::index::get_all_songs(limit, offset).expect("Failed to get all songs");
    Ok(tracks)
}
