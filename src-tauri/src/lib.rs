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
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_decorum::init())
        .plugin(tauri_plugin_clipboard_manager::init())
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
            conversation::get_assistant_conversation,
            conversation::delete_message,
            knowledge_base::get_all_knowledge_base,
            knowledge_base::get_knowledge_base_files,
            knowledge_base::new_knowledge_base,
            knowledge_base::add_file_to_knowledge_base,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn setup(app: AppHandle) -> Result<()> {
    let app_data_path = path::get_data_dir(&app)?;
    let state = app.state::<AppState>();
    // 不要调换下面两行的顺序
    let embedding_db_conn = dbaccess::embed::get_embedding_db_conn(&app_data_path).await?;
    let db_conn = dbaccess::get_db_conn(&app_data_path).await?;
    *state.db.write().await = db_conn;
    *state.embedding_db.write().await = Some(embedding_db_conn);
    Ok(())
}
