use tauri::Manager;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetParent;
use std::ffi::c_void;
use serde::Serialize;
use windows::Win32::Graphics::Dwm::{
    DWMSBT_AUTO,
    DWMSBT_NONE,
    DWMSBT_MAINWINDOW,
    DWMSBT_TRANSIENTWINDOW,
    DWMSBT_TABBEDWINDOW,
    DWMWA_SYSTEMBACKDROP_TYPE,
    DwmGetWindowAttribute,
    DwmSetWindowAttribute
};
use super::database::{
    set_config_value,
    connection
};

pub fn get_system_version() -> u32 {
    #[cfg(windows)]
    {
        use winreg::RegKey;
        use winreg::enums::*;

        let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
        let current_version = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
            .expect("Failed to open registry key");

        let build_number: String = current_version.get_value("CurrentBuild")
            .expect("Failed to get build number");
        
        build_number.parse().unwrap_or(0)
    }

    #[cfg(not(windows))]
    (0)
}

#[derive(Serialize)]
pub struct MaterialInfo {
    pub id: i32,
    pub name: &'static str,
}

#[cfg(windows)]
pub fn get_supported_materials() -> Vec<MaterialInfo> {
    let mut materials = vec![MaterialInfo {
        id: DWMSBT_AUTO.0,
        name: "Default",
    }];

    let build_number = get_system_version();

    if build_number >= 22000 {
        materials.push(MaterialInfo { id: DWMSBT_NONE.0, name: "None (No backdrop)" });
        materials.push(MaterialInfo { id: DWMSBT_MAINWINDOW.0, name: "Mica" });
        materials.push(MaterialInfo { id: DWMSBT_TRANSIENTWINDOW.0, name: "Acrylic" });
        materials.push(MaterialInfo { id: DWMSBT_TABBEDWINDOW.0, name: "Tabbed (MicaAlt)" });
    }

    materials
}

#[cfg(windows)]
pub fn get_current_materials(app: &tauri::AppHandle) -> Vec<MaterialInfo> {
    let materials = get_supported_materials();
    let hwnd = get_window_hwnd(app.clone()).unwrap();

    let mut current_materials = vec![];
    for material in materials {
        let mut material_id = 0;
        unsafe {
            let _ = DwmGetWindowAttribute(
                HWND(hwnd as *mut c_void), 
                DWMWA_SYSTEMBACKDROP_TYPE, 
                (&mut material_id as *mut i32).cast::<c_void>(), 
                std::mem::size_of::<i32>() as u32);
        }
        if material_id == material.id {
            current_materials.push(material);
        }
    }
    current_materials
}

#[cfg(windows)]
pub fn set_window_material(app: &tauri::AppHandle, material: i32) -> u32 {
    let hwnd = get_window_hwnd(app.clone()).unwrap();
    let mut material_id = material;
    unsafe {
        let _ = DwmSetWindowAttribute(
            HWND(hwnd as *mut c_void), 
            DWMWA_SYSTEMBACKDROP_TYPE, 
            (&mut material_id as *mut i32).cast::<c_void>(), 
            std::mem::size_of::<i32>() as u32);
    }
    _ = set_config_value(&connection(), "material", &material_id.to_string());
    material_id.try_into().unwrap()
}

#[cfg(windows)]
pub fn get_window_hwnd(app: tauri::AppHandle) -> Option<isize> {
    let window = app.get_webview_window("main")?;

    if let Ok(handle) = window.window_handle() {
        match handle.as_raw() {
            RawWindowHandle::Win32(h) => {
                let mut current = HWND(isize::from(h.hwnd) as *mut c_void);
                unsafe {
                    loop {
                        match GetParent(current) {
                            Ok(parent) if !parent.0.is_null() => {
                                current = parent;
                            }
                            _ => break,
                        }
                    }
                }
                return Some(current.0 as isize);
            }
            _ => {}
        }
    }
    None
}

