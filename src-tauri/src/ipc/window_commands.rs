use crate::app::window;
use crate::app::window::MaterialInfo;

#[tauri::command]
pub fn get_system_version() -> u32 {
    window::get_system_version()
}

#[tauri::command]
pub fn get_supported_window_materials() -> Vec<MaterialInfo> {
    window::get_supported_materials()
}

#[tauri::command]
pub fn get_current_window_materials(app: tauri::AppHandle) -> Vec<MaterialInfo> {
    window::get_current_materials(&app)
}

#[tauri::command]
pub fn set_window_material(app: tauri::AppHandle, material: i32) -> u32 {
    window::set_window_material(&app, material)
}

#[tauri::command]
pub fn get_window_hwnd(app: tauri::AppHandle) -> Option<isize> {
    window::get_window_hwnd(app)
}
