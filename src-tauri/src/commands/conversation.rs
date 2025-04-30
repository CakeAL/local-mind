use chrono::{DateTime, Utc};
use ollama_rust_api::model::{
    chat::{ChatRequestParameters, Message},
    embedding::EmbedRequestParameters,
};
use sea_orm::DbConn;
use serde::Serialize;
use tauri::ipc::Channel;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{
    dbaccess::{self, embed::SearchResult},
    models::{app_state::AppState, assistant::AssistantInfo, conversation},
};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum ChatResponseEvent {
    #[serde(rename_all = "camelCase")]
    Started { user_message: conversation::Model },
    #[serde(rename_all = "camelCase")]
    ReferenceContent { search_result: Vec<SearchResult> },
    #[serde(rename_all = "camelCase")]
    Progress {
        model: String,
        create_at: DateTime<Utc>,
        content: String,
    },
    #[serde(rename_all = "camelCase")]
    Finished {
        assistant_message: conversation::Model,
    },
}

pub async fn generate_message(
    state: &tauri::State<'_, AppState>,
    db: &DbConn,
    assistant_uuid: Uuid,
    context_num: i64,
    model: String,
    on_event: Channel<ChatResponseEvent>,
    message_id: Option<i64>,
    knowledge_base: Option<String>,
) -> Result<(), String> {
    // 取出 message_id 之前（含）最近 context_num 条消息（作为上下文消息）
    let mut nearest_messages = dbaccess::conversation::select_message_by_uuid(
        db,
        assistant_uuid,
        Some(context_num as u64),
        message_id,
    )
    .await
    .map_err(|e| e.to_string())?
    .into_iter()
    .map(|m| Message {
        role: m.role.into(),
        content: m.content,
        images: None,
    })
    .collect::<Vec<Message>>();
    // 执行相似度查询
    let ollama = state.ollama.read().await;
    let search_result = if let Some(knowledge_base) = knowledge_base {
        let knowledge_base =
            dbaccess::knowledge_base::select_knowledge_base_by_name(db, &knowledge_base)
                .await
                .map_err(|e| e.to_string())?;
        let query_embedding = ollama
            .embedding(&EmbedRequestParameters {
                model: knowledge_base.model,
                input: vec![nearest_messages
                    .first()
                    .map(|m| m.content.clone())
                    .unwrap_or_default()],
            })
            .await
            .map_err(|e| e.to_string())?;
        if let Some(query_embedding) = query_embedding.embeddings.first() {
            let embedding_db = state.get_embedding_db().await?;
            let search_result = dbaccess::embed::search_similar_embeddings(
                &embedding_db,
                &knowledge_base.name,
                query_embedding,
            )
            .await
            .map_err(|e| e.to_string())?;
            on_event
                .send(ChatResponseEvent::ReferenceContent {
                    search_result: search_result.clone(),
                })
                .map_err(|e| e.to_string())?;
            Some(search_result)
        } else {
            None
        }
    } else {
        None
    };
    // 将相似度查询结果加入到 nearest_messages first中
    if let Some(search_result) = search_result.as_ref() {
        if let Some(message) = nearest_messages.first_mut() {
            let new_content = search_result
                .iter()
                .map(|sr| sr.content.clone()) // 克隆content以避免借用问题
                .collect::<Vec<String>>()
                .join("\n");

            message.content = format!("{}\n{}", new_content, message.content);
        }
    }
    // 取出该 assistant 配置信息
    let para_value = dbaccess::assistant::select_assistant_config(db, assistant_uuid)
        .await
        .map_err(|e| e.to_string())?;
    // 向 Ollama 发送消息
    let mut chat_stream = ollama
        .chat(&ChatRequestParameters {
            model,
            messages: nearest_messages,
            options: Some(para_value),
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
        Some(chat_response) => {
            let assistant_message_model = if let Some(message_id) = message_id {
                // 说明是 Regenerate
                dbaccess::conversation::update_assistant_message(
                    db,
                    message_id,
                    chat_response,
                    final_content,
                    search_result,
                )
                .await
            } else {
                dbaccess::conversation::insert_assistant_message(
                    db,
                    assistant_uuid,
                    chat_response,
                    final_content,
                    search_result,
                )
                .await
            }
            .map_err(|e| e.to_string())?;
            on_event
                .send(ChatResponseEvent::Finished {
                    assistant_message: assistant_message_model,
                })
                .map_err(|e| e.to_string())?;
            Ok(())
        }
        None => Err("Generate Reply Failed due to Ollama inner Error. Please Retry.".into()),
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
        id: _,
        uuid: assistant_uuid,
        name: _,
        model,
        context_num,
        knowledge_base,
    } = assistant_info;
    let db = state.db.read().await;
    // 插入用户消息至数据库
    let user_message_model =
        dbaccess::conversation::insert_user_message(&db, assistant_uuid, model.to_owned(), content)
            .await
            .map_err(|e| e.to_string())?;
    on_event
        .send(ChatResponseEvent::Started {
            user_message: user_message_model,
        })
        .map_err(|e| e.to_string())?;
    // 让大模型生成新的消息
    generate_message(
        &state,
        &db,
        assistant_uuid,
        context_num,
        model,
        on_event,
        None,
        knowledge_base,
    )
    .await
}

#[tauri::command]
pub async fn regenerate_assistant_message(
    state: tauri::State<'_, AppState>,
    assistant_info: AssistantInfo,
    message_id: i64,
    on_event: Channel<ChatResponseEvent>,
) -> Result<(), String> {
    let AssistantInfo {
        id: _,
        uuid: assistant_uuid,
        name: _,
        model,
        context_num,
        knowledge_base,
    } = assistant_info;
    let db = state.db.read().await;
    // 让大模型生成新的消息，这里应该根据 message_id 之前的消息来生成消息
    generate_message(
        &state,
        &db,
        assistant_uuid,
        context_num,
        model,
        on_event,
        Some(message_id),
        knowledge_base,
    )
    .await
}

#[tauri::command]
pub async fn get_assistant_conversation(
    state: tauri::State<'_, AppState>,
    assistant_uuid: Uuid,
) -> Result<Vec<conversation::Model>, String> {
    let db = state.db.read().await;
    dbaccess::conversation::select_message_by_uuid(&db, assistant_uuid, None, None)
        .await
        .map_err(|e| e.to_string())
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
