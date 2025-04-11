use ollama_rust_api::model::parameter::Parameter;
use sea_orm::ActiveModelTrait;
use sea_orm::{ActiveValue::Set, ConnectionTrait, DbConn, Schema, Statement};
use uuid::Uuid;

use crate::entities::assistant;

/// 用来创建 assistant 表
pub async fn create_assistant_table(db: &DbConn) -> Result<(), sea_orm::DbErr> {
    // 检查表是否存在
    let exec_res = db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Sqlite,
            "PRAGMA table_info(assistant)",
        ))
        .await?;
    if exec_res.is_empty() {
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let statement = builder.build(&schema.create_table_from_entity(assistant::Entity));
        db.execute(statement).await?;
        tracing::info!("Table assistant has created.");
    }
    Ok(())
}

/// 新建一个助手
pub async fn new_assistant(db: &DbConn) -> Result<assistant::Model, sea_orm::DbErr> {
    let new_assistant = assistant::ActiveModel {
        name: Set("Default Assistant".to_string()),
        uuid: Set(Uuid::new_v4()),
        parameter: Set(serde_json::json!(Parameter::default())),
        ..Default::default()
    };
    let new_assistant: assistant::Model = new_assistant.insert(db).await?;
    Ok(new_assistant)
}
