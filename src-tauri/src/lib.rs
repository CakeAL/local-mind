use anyhow::Result;
use entities::app_state::AppState;
use tauri::{async_runtime::spawn, AppHandle, Manager};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utils::{path, window};

pub mod dbaccess;
pub mod entities;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::filter::filter_fn(|metadata| {
            metadata.level() > &LevelFilter::TRACE
        }))
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_decorum::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![])
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
