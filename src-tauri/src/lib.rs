use entities::app_state::AppState;
use utils::window;

pub mod utils;
pub mod entities;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_decorum::init())
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let main_window = window::build_main_window(app)?;
            window::set_titlebar_style(&main_window)?;
            window::set_background_effect(&main_window)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
