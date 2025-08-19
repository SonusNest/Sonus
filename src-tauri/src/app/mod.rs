pub mod init;

pub mod database;

pub mod window;
pub use window::{
    get_system_version, 
    get_supported_materials, 
    get_current_materials,
    set_window_material,
    get_window_hwnd
};
