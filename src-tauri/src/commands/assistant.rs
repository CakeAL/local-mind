use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime,
};
use uuid::Uuid;

use crate::{
    dbaccess,
    models::{app_state::AppState, assistant::AssistantInfo},
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("assistant_command")
        .invoke_handler(tauri::generate_handler![
            new_assistant,
            get_all_assistant,
            get_assistant_config,
            update_assistant_config,
            delete_assistant
        ])
        .build()
}

#[tauri::command]
pub async fn new_assistant(state: tauri::State<'_, AppState>) -> Result<AssistantInfo, String> {
    let db = state.db.read().await;
    let new_assistant = dbaccess::assistant::insert_assistant(&db)
        .await
        .map_err(|e| e.to_string())?;
    let assistant_info: AssistantInfo = new_assistant.into();
    Ok(assistant_info)
}

#[tauri::command]
pub async fn get_all_assistant(
    state: tauri::State<'_, AppState>,
) -> Result<Vec<AssistantInfo>, String> {
    let db = state.db.read().await;
    dbaccess::assistant::select_all_assistant(&db)
        .await
        .map(|v| v.into_iter().map(AssistantInfo::from).collect())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_assistant_config(
    state: tauri::State<'_, AppState>,
    uuid: Uuid,
) -> Result<serde_json::Value, String> {
    let db = state.db.read().await;
    dbaccess::assistant::select_assistant_config(&db, uuid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_assistant_config(
    state: tauri::State<'_, AppState>,
    uuid: Uuid,
    para: serde_json::Value,
    context_num: Option<u64>,
) -> Result<(), String> {
    let db = state.db.read().await;
    let _ = dbaccess::assistant::update_assistant_config(&db, uuid, para, context_num)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn delete_assistant(
    state: tauri::State<'_, AppState>,
    uuid: Uuid,
) -> Result<bool, String> {
    let db = state.db.read().await;
    // a首先删除该 assistant 所有对话
    let _ = dbaccess::conversation::delete_all_message(&db, uuid).await;
    dbaccess::assistant::delete_assistant(&db, uuid)
        .await
        .map_err(|e| e.to_string())
}
