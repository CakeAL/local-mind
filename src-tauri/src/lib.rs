use anyhow::Result;
use commands::*;
use models::app_state::AppState;
use tauri::{async_runtime::spawn, AppHandle, Manager};
use utils::{path, window};

pub mod commands;
pub mod dbaccess;
pub mod models;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_decorum::init())
        .manage(AppState::default())
        .setup(|app| {
            let main_window = window::build_main_window(app)?;
            window::set_titlebar_style(&main_window)?;
            window::set_background_effect(&main_window)?;
            spawn(setup(app.handle().clone()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            assistant::list_models,
            assistant::new_assistant,
            assistant::get_all_assistant,
            assistant::get_assistant_config,
            assistant::update_assistant_config,
            assistant::delete_assistant,
            conversation::user_send_message,
            conversation::regenerate_assistant_message,
            conversation::delete_message
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: AppHandle) -> Result<()> {
    let app_data_path = path::get_data_dir(&app)?;
    let app_state = app.state::<AppState>();
    *app_state.db.write().await = dbaccess::get_db_conn(&app_data_path).await?;
    Ok(())
}
