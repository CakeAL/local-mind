use std::path::PathBuf;

use anyhow::Result;
use std::fs::create_dir;
use tauri::{AppHandle, Manager};

pub fn get_data_dir(app: &AppHandle) -> Result<PathBuf> {
    let app_data_dir = app.path().app_data_dir()?;
    if !app_data_dir.exists() {
        create_dir(&app_data_dir)?;
    }
    Ok(app_data_dir)
}
