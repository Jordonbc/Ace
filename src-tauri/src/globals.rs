use log::{trace, info};
use serde::{Deserialize, Serialize};
use tauri::Manager;
use std::fmt;

use std::path::PathBuf;
use std::sync::Mutex;
use directories::ProjectDirs;

use crate::error::LogOnError;

/*#[derive(Debug, Clone)]
pub struct Window {
    pub window: tauri::Window
}*/

pub static MAIN_HANDLE: Mutex<Option<tauri::AppHandle>> = Mutex::new(None);
pub static MAIN_WINDOW: Mutex<Option<tauri::Window>> = Mutex::new(None);
pub static CONFIG: Mutex<Option<ConfigTemplate>> = Mutex::new(None);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfigTemplate {
}

impl fmt::Display for ConfigTemplate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

lazy_static! {
    pub static ref CONFIG_LOCATION: PathBuf = {
        let dirs = ProjectDirs::from("dev", "", "NEAV1E_rust");
        match dirs {
        Some(_) => return dirs.unwrap().config_dir().to_path_buf().join("config.json"),
        None => todo!(),
        }
    };
}

pub fn get_window(window_label: &str) -> Result<tauri::Window, Box<dyn std::error::Error>> {
    trace!("Getting current window reference");

    let main_handle = MAIN_HANDLE.lock().map_err(|_| "Failed to acquire lock")?;

    let window = main_handle
        .as_ref()
        .ok_or("Main handle is None")?
        .get_window(window_label)
        .ok_or_else(|| format!("No window found with label: {}", window_label))
        .log_on_err()?;

    Ok(window)
}

pub fn get_app_handle() -> Result<tauri::AppHandle, Box<dyn std::error::Error>> {
    let main_window = get_main_window().log_on_err()?;
    Ok(main_window.app_handle())
}

pub fn get_main_window() -> Result<tauri::Window, Box<dyn std::error::Error>>  {
    get_window("main")
}

pub fn set_main_window(new_window: tauri::Window) -> Result<(), Box<dyn std::error::Error>> {
    trace!("Setting main Window: {}", new_window.label());
    let mut main_window = MAIN_WINDOW.lock().map_err(|_| "Failed to acquire lock").log_on_err()?;
    *main_window = Some(new_window);

    Ok(())
}


pub fn update_frontend() -> Result<(), Box<dyn std::error::Error>> {
    info!("Updating frontend!");

    let main_window = get_main_window().log_on_err()?;

    main_window.emit_all("update_frontend", true)
        .log_on_err().map_err(|_| "Error Sending update event to frontend!".into())
}