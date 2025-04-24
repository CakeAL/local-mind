use ollama_rust_api::model::parameter::Parameter;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter, QueryOrder, QuerySelect,
};
use sea_orm::{ActiveValue::Set, ConnectionTrait, DbConn, Schema, Statement};
use uuid::Uuid;

use crate::models::assistant;

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
pub async fn insert_assistant(
    db: &DbConn,
    model: String,
) -> Result<assistant::Model, sea_orm::DbErr> {
    let new_assistant = assistant::ActiveModel {
        name: Set("Default Assistant".to_string()),
        model: Set(model),
        uuid: Set(Uuid::new_v4()),
        parameter: Set(serde_json::json!(Parameter {
            temperature: Some(1.0),
            top_p: Some(1.0),
            ..Default::default()
        })),
        context_num: Set(5),
        ..Default::default()
    };
    let new_assistant: assistant::Model = new_assistant.insert(db).await?;
    Ok(new_assistant)
}

/// 返回所有助手
pub async fn select_all_assistant(
    db: &DbConn,
) -> Result<Vec<assistant::AssistantInfo>, sea_orm::DbErr> {
    let assistants = assistant::Entity::find()
        .select_only()
        .columns([
            assistant::Column::Id,
            assistant::Column::Uuid,
            assistant::Column::Name,
            assistant::Column::Model,
            assistant::Column::ContextNum,
        ])
        .order_by_asc(assistant::Column::Id)
        .into_model::<assistant::AssistantInfo>()
        .all(db)
        .await?;
    Ok(assistants)
}

/// 返回某个助手的配置
pub async fn select_assistant_config(db: &DbConn, uuid: Uuid) -> Result<Parameter, sea_orm::DbErr> {
    let v = assistant::Entity::find()
        .filter(assistant::Column::Uuid.eq(uuid))
        // .select_only()
        // .columns([assistant::Column::Parameter])
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No such uuid to assistant".into(),
        ))?;
    Ok(serde_json::from_value(v.parameter).unwrap_or_default())
}

/// 更新某个助手的配置
pub async fn update_assistant_config(
    db: &DbConn,
    uuid: Uuid,
    name: String,
    para: Parameter,
    context_num: Option<i64>,
) -> Result<assistant::Model, sea_orm::DbErr> {
    let assistant = assistant::Entity::find()
        .filter(assistant::Column::Uuid.eq(uuid))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No such uuid to assistant".into(),
        ))?;
    let mut assistant: assistant::ActiveModel = assistant.into();
    assistant.parameter = Set(serde_json::json!(para));
    assistant.name = Set(name);
    if let Some(context_num) = context_num {
        assistant.context_num = Set(context_num);
    }
    let updated_assistant = assistant.update(db).await?;
    Ok(updated_assistant)
}

/// 删除某个助手
pub async fn delete_assistant(db: &DbConn, uuid: Uuid) -> Result<bool, sea_orm::DbErr> {
    let assistant = assistant::Entity::find()
        .filter(assistant::Column::Uuid.eq(uuid))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No such uuid to assistant".into(),
        ))?;
    let res = assistant.delete(db).await?;
    Ok(res.rows_affected == 1)
}
