// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // window commands
            ipc::get_system_version,
            ipc::get_supported_window_materials,
            ipc::get_current_window_materials,
            ipc::set_window_material,
            ipc::get_window_hwnd,
            // task queue commands
            ipc::start_directory_scan,
            ipc::get_task_stats,
            ipc::register_task_listener,
            // library commands
            ipc::get_all_songs,
            // player commands
            ipc::play_to_playlist,
            ipc::play_from,
            ipc::play,
            ipc::pause,
            ipc::resume,
            ipc::stop,
            ipc::set_volume,
            ipc::shutdown_player,
            ipc::next_track,
            ipc::previous_track,
            ipc::insert_track_at,
            ipc::insert_track_after_current,
            ipc::add_track_to_end,
            ipc::move_track,
            ipc::remove_track,
            ipc::clear_playlist,
            ipc::overwrite_playlist,
            ipc::set_play_mode,
            ipc::get_current_index,
            ipc::set_and_play_index
        ])
        .setup(|app| {
            // init app
            let app_handle = app.handle();
            crate::app::init::init(&app_handle);
            init_task_queue(app.handle().clone())?;
            // init player
            let shared_state = core::player::state::new_shared_state();
            let player_controller = new_shared_player_controller(shared_state, app.handle().clone())
                .expect("Failed to initialize player controller");
            app.manage(player_controller);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


pub mod ipc;
pub mod app;
pub mod core;
pub mod utils;

use tauri::Manager;
use core::task_queue::tauri_integration::init_task_queue;
use crate::core::controller::{new_shared_player_controller, PlayerController};
