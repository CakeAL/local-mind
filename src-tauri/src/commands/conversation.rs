use chrono::{DateTime, Utc};
use ollama_rust_api::model::{
    chat::{ChatRequestParameters, Message},
    parameter::Parameter,
};
use sea_orm::DbConn;
use serde::Serialize;
use tauri::ipc::Channel;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    dbaccess,
    models::{app_state::AppState, assistant::AssistantInfo},
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum ChatResponseEvent {
    #[serde(rename_all = "camelCase")]
    Started { user_message: serde_json::Value },
    #[serde(rename_all = "camelCase")]
    Progress {
        model: String,
        create_at: DateTime<Utc>,
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        assistant_message: serde_json::Value,
    },
}

pub async fn generate_message(
    state: &tauri::State<'_, AppState>,
    db: &DbConn,
    assistant_uuid: Uuid,
    context_num: u64,
    model: String,
    on_event: Channel<ChatResponseEvent>,
) -> Result<(), String> {
    // 取出最近 context_num 条消息（作为上下文消息）
    let nearest_messages =
        dbaccess::conversation::select_message_by_uuid(db, assistant_uuid, Some(context_num))
            .await
            .map_err(|e| e.to_string())?
            .into_iter()
            .map(|m| Message {
                role: m.role.into(),
                content: m.content,
                images: None,
            })
            .collect::<Vec<Message>>();
    // 取出该 assistant 配置信息
    let para_value = dbaccess::assistant::select_assistant_config(db, assistant_uuid)
        .await
        .map_err(|e| e.to_string())?;
    let para = serde_json::from_value::<Parameter>(para_value).map_err(|e| e.to_string())?;
    // 向 Ollama 发送消息
    let ollama = state.ollama.read().await;
    let mut chat_stream = ollama
        .chat(&ChatRequestParameters {
            model,
            messages: nearest_messages,
            options: Some(para),
        })
        .await
        .map_err(|e| e.to_string())?;
    // 读取流
    let mut final_content = String::new();
    let mut final_res = None;
    while let Some(Ok(res)) = chat_stream.next().await {
        if res.done {
            final_res = Some(res);
        } else {
            // 发送到前端
            final_content.push_str(&res.message.content);
            on_event
                .send(ChatResponseEvent::Progress {
                    model: res.model,
                    create_at: res.created_at,
                    content: res.message.content,
                })
                .map_err(|e| e.to_string())?;
        }
    }
    // 如果 final_res 为 None，说明 Ollama 没有成功生成完整的对话
    match final_res {
        Some(res) => {
            let assistant_message_model = dbaccess::conversation::insert_assistant_message(
                db,
                assistant_uuid,
                res,
                final_content,
            )
            .await
            .map_err(|e| e.to_string())?;
            on_event
                .send(ChatResponseEvent::Finished {
                    assistant_message: serde_json::json!(assistant_message_model),
                })
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => Err("Generate Reply Failed due to Ollama inner Error.".into()),
    }
}

#[tauri::command]
pub async fn user_send_message(
    state: tauri::State<'_, AppState>,
    assistant_info: AssistantInfo,
    content: String,
    on_event: Channel<ChatResponseEvent>,
) -> Result<(), String> {
    let AssistantInfo {
        uuid: assistant_uuid,
        name: _,
        model,
        context_num,
    } = assistant_info;
    let db = state.db.read().await;
    // 插入用户消息至数据库
    let user_message_model =
        dbaccess::conversation::insert_user_message(&db, assistant_uuid, model.to_owned(), content)
            .await
            .map_err(|e| e.to_string())?;
    on_event
        .send(ChatResponseEvent::Started {
            user_message: serde_json::json!(user_message_model),
        })
        .map_err(|e| e.to_string())?;
    // 让大模型生成新的消息
    generate_message(&state, &db, assistant_uuid, context_num, model, on_event).await
}

#[tauri::command]
pub async fn regenerate_assistant_message(
    state: tauri::State<'_, AppState>,
    assistant_info: AssistantInfo,
    message_id: i64,
    on_event: Channel<ChatResponseEvent>,
) -> Result<(), String> {
    let AssistantInfo {
        uuid: assistant_uuid,
        name: _,
        model,
        context_num,
    } = assistant_info;
    let db = state.db.read().await;
    // 删除当前消息
    delete_message(state.clone(), message_id).await?;
    // 让大模型生成新的消息
    generate_message(&state, &db, assistant_uuid, context_num, model, on_event).await
}

#[tauri::command]
pub async fn delete_message(
    state: tauri::State<'_, AppState>,
    message_id: i64,
) -> Result<(), String> {
    let db = state.db.read().await;
    if !dbaccess::conversation::delete_message(&db, message_id)
        .await
        .map_err(|e| e.to_string())?
    {
        return Err("No such message.".into());
    }
    Ok(())
}
