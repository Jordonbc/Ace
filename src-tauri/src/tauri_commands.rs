use crate::{globals, error::LogOnError};

#[tauri::command]
pub async fn open_source_dialog() {
    let app_handle = globals::get_app_handle().log_on_err().expect("App handle invalid!");

    tauri::WindowBuilder::new(
        &app_handle,
        "open_source", /* the unique window label */
        tauri::WindowUrl::App("index.html".into())
      ).build().log_on_err().unwrap();
}
