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
        ])
        .setup(|app| {
            let app_handle = app.handle();
            crate::app::init::init(&app_handle);
            init_task_queue(app.handle().clone())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


pub mod ipc;
pub mod app;
pub mod core;
pub mod utils;
mod play;

use core::task_queue::tauri_integration::init_task_queue;
