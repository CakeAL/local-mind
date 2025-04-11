use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("assistant_command")
        .invoke_handler(tauri::generate_handler![
            new_assistant
        ])
        .build()
}

#[tauri::command]
pub async fn new_assistant() -> Result<(), String> {
    Ok(())
}