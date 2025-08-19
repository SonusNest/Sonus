// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tracing::{info};
use tracing_subscriber::fmt;

fn main() {
    // init logger
    fmt::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("Sonus App is runing...");

    sonus_lib::run()
}
