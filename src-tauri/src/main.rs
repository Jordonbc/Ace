#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

#[macro_use]
extern crate lazy_static;

use log;
use log4rs;
use std::process::Command;
use std::env;
use tauri::{Manager, LogicalSize};

mod globals;
mod tauri_commands;
mod config;
mod error;

use crate::error::LogOnError;

/*
*   This will be used to kill processes of ffmpeg
*/
#[allow(dead_code)]
fn kill_process_tree(pid: u32) {
    if cfg!(target_os = "windows") {
        log::info!("Killing via taskkill...");
        Command::new("taskkill")
            .args(["/pid", &pid.to_string(), "/T", "/F"])
            .output()
            .expect("Failed to kill process tree");
    } else {
        log::info!("Killing via pkill...");
        Command::new("pkill")
            .arg("-P")
            .arg(pid.to_string())
            .output()
            .expect("Failed to kill process tree");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    log4rs::init_file("logging_config.yaml", Default::default()).log_on_err()?;

    log::trace!("Hello, World! I'm awake!");
    config::read_config_file();
    
    tauri::Builder::default()
        .setup(|app|{
            *globals::MAIN_HANDLE.lock().unwrap() = Some(app.app_handle());

            if let Some(window) = app.get_window("main") {
                globals::set_main_window(window)?;
            }
            
            globals::get_main_window().log_on_err()?.set_min_size(Some(LogicalSize { width: 780.0, height: 550.0 })).log_on_err()?;

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            tauri_commands::open_source_dialog
        ])
        .run(tauri::generate_context!())?;

    Ok(())
    
}
