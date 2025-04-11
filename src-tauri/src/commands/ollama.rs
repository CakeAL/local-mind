use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("ollama_plugin")
        .invoke_handler(tauri::generate_handler![])
        .build()
}
