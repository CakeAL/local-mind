use crate::models::app_state::AppState;

#[tauri::command]
pub async fn set_ollama_url(
    app: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    url: String,
) -> Result<(), String> {
    state
        .ollama
        .write()
        .await
        .change_host(&url)
        .map_err(|e| e.to_string())?;
    state
        .setting
        .write()
        .await
        .set_ollama_url(url, &app)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_ollama_url(state: tauri::State<'_, AppState>) -> Result<String, String> {
    Ok(state.setting.read().await.ollama_url.clone())
}

#[tauri::command]
pub async fn check_ollama_online(state: tauri::State<'_, AppState>) -> Result<bool, String> {
    Ok(state.ollama.read().await.check_online().await)
}
