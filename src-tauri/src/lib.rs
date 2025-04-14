use anyhow::Result;
use models::app_state::AppState;
use tauri::{async_runtime::spawn, AppHandle, Manager};
use utils::{path, window};

pub mod dbaccess;
pub mod models;
pub mod utils;
pub mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_decorum::init())
        .plugin(commands::conversation::init())
        .plugin(commands::assistant::init())
        .manage(AppState::default())
        .setup(|app| {
            let main_window = window::build_main_window(app)?;
            window::set_titlebar_style(&main_window)?;
            window::set_background_effect(&main_window)?;
            spawn(setup(app.handle().clone()));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: AppHandle) -> Result<()> {
    let app_data_path = path::get_data_dir(&app)?;
    let app_state = app.state::<AppState>();
    *app_state.db.write().await = dbaccess::get_db_conn(&app_data_path).await?;
    Ok(())
}
