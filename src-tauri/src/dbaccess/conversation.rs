use chrono::Utc;
use ollama_rust_api::model::chat::ChatResponse;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbBackend, DbConn,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, Schema, Statement,
};
use uuid::Uuid;

use crate::models::conversation;

/// 用来创建 conversation 表
pub async fn create_conversation_table(db: &DbConn) -> Result<(), sea_orm::DbErr> {
    // 检查表是否存在
    let exec_res = db
        .query_all(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA table_info(conversation)",
        ))
        .await?;
    if exec_res.is_empty() {
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let statement = builder.build(&schema.create_table_from_entity(conversation::Entity));
        db.execute(statement).await?;
        tracing::info!("Table conversation has created.");
    }
    Ok(())
}

/// 插入用户发送的一段消息
pub async fn insert_user_message(
    db: &DbConn,
    assistant_uuid: Uuid,
    model: String,
    content: String,
) -> Result<conversation::Model, sea_orm::DbErr> {
    let new_message = conversation::ActiveModel {
        assistant_uuid: Set(assistant_uuid),
        model: Set(model),
        created_at: Set(Utc::now()),
        content: Set(content),
        role: Set(conversation::RoleType::User),
        total_duration: Set(0),
        load_duration: Set(0),
        prompt_eval_count: Set(0),
        prompt_eval_duration: Set(0),
        eval_count: Set(0),
        eval_duration: Set(0),
        ..Default::default()
    };
    let new_message = new_message.insert(db).await?;
    Ok(new_message)
}

/// 插入模型回复的消息
pub async fn insert_assistant_message(
    db: &DbConn,
    assistant_uuid: Uuid,
    chat_response: ChatResponse,
    content: String,
) -> Result<conversation::Model, sea_orm::DbErr> {
    let ChatResponse {
        model,
        created_at,
        message: _,
        done: _,
        total_duration,
        load_duration,
        prompt_eval_count,
        prompt_eval_duration,
        eval_count,
        eval_duration,
        context: _,
    } = chat_response;
    let new_message = conversation::ActiveModel {
        assistant_uuid: Set(assistant_uuid),
        model: Set(model),
        created_at: Set(created_at),
        content: Set(content),
        role: Set(conversation::RoleType::Assistant), // 临时就先这样了
        total_duration: Set(total_duration.unwrap_or_default()),
        load_duration: Set(load_duration.unwrap_or_default()),
        prompt_eval_count: Set(prompt_eval_count.unwrap_or_default()),
        prompt_eval_duration: Set(prompt_eval_duration.unwrap_or_default()),
        eval_count: Set(eval_count.unwrap_or_default()),
        eval_duration: Set(eval_duration.unwrap_or_default()),
        ..Default::default()
    };
    let new_message = new_message.insert(db).await?;
    Ok(new_message)
}

/// 查找某 assistant 的全部(最近 context_num)对话
pub async fn select_message_by_uuid(
    db: &DbConn,
    assistant_uuid: Uuid,
    context_num: Option<u64>,
) -> Result<Vec<conversation::Model>, sea_orm::DbErr> {
    let messages = conversation::Entity::find()
        .filter(conversation::Column::AssistantUuid.eq(assistant_uuid))
        .order_by_desc(conversation::Column::CreatedAt) // 按创建时间倒序排序
        .limit(context_num) // 限制返回的记录数
        .all(db) // 获取所有匹配的记录
        .await?;
    Ok(messages)
}

/// 删除某条消息
pub async fn delete_message(db: &DbConn, id: i64) -> Result<bool, sea_orm::DbErr> {
    let res = conversation::Entity::delete_by_id(id).exec(db).await?;
    Ok(res.rows_affected == 1)
}

/// 删除某个助手相关的所有对话
pub async fn delete_all_message(db: &DbConn, assistant_uuid: Uuid) -> Result<bool, sea_orm::DbErr> {
    let _res = conversation::Entity::delete_many()
        .filter(conversation::Column::AssistantUuid.eq(assistant_uuid))
        .exec(db)
        .await?;
    Ok(true)
}
