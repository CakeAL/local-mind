use crate::models::knowledge_base;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, ConnectionTrait, DbBackend, DbConn,
    EntityTrait, ModelTrait, QueryFilter, Schema, Statement,
};

/// 用来创建 knowledge_base 表
pub async fn create_knowledge_base_table(db: &DbConn) -> Result<(), sea_orm::DbErr> {
    // 检查表是否存在
    let exec_res = db
        .query_all(Statement::from_string(
            DbBackend::Sqlite,
            "PRAGMA table_info(knowledge_base)",
        ))
        .await?;
    if exec_res.is_empty() {
        let builder = db.get_database_backend();
        let schema = Schema::new(builder);
        let statement = builder.build(&schema.create_table_from_entity(knowledge_base::Entity));
        db.execute(statement).await?;
        tracing::info!("Table knowledge_base has created.");
    }
    Ok(())
}

/// 新建知识库
pub async fn insert_knowledge_base(
    db: &DbConn,
    name: String,
    model: String,
) -> Result<knowledge_base::Model, sea_orm::DbErr> {
    let new_knowledge_base = knowledge_base::ActiveModel {
        name: Set(name),
        model: Set(model),
        ..Default::default()
    };
    let new_knowledge_base = new_knowledge_base.insert(db).await?;
    Ok(new_knowledge_base)
}

/// 向知识库新增文件
pub async fn update_knowledge_base_files(
    db: &DbConn,
    name: String,
    file_paths: Vec<String>,
) -> Result<knowledge_base::Model, sea_orm::DbErr> {
    let knowledge_base = knowledge_base::Entity::find()
        .filter(knowledge_base::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No Such Knowledge Base".into(),
        ))?;
    let mut stored_paths = serde_json::from_value::<Vec<String>>(knowledge_base.file_paths.clone())
        .unwrap_or_default();
    let mut knowledge_base: knowledge_base::ActiveModel = knowledge_base.into();
    stored_paths.extend(file_paths);
    knowledge_base.file_paths = Set(serde_json::json!(stored_paths));
    let knowledge_base = knowledge_base.update(db).await?;
    Ok(knowledge_base)
}

/// 向知识库删除某文件
pub async fn update_knowledge_base_file_delete(
    db: &DbConn,
    name: String,
    file_path: String,
) -> Result<knowledge_base::Model, sea_orm::DbErr> {
    let knowledge_base = knowledge_base::Entity::find()
        .filter(knowledge_base::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No Such Knowledge Base".into(),
        ))?;
    let stored_paths = serde_json::from_value::<Vec<String>>(knowledge_base.file_paths.clone())
        .unwrap_or_default();
    let mut knowledge_base: knowledge_base::ActiveModel = knowledge_base.into();
    let updated_paths = stored_paths
        .into_iter()
        .filter(|path| *path != file_path)
        .collect::<Vec<String>>();
    knowledge_base.file_paths = Set(serde_json::json!(updated_paths));
    let knowledge_base = knowledge_base.update(db).await?;
    Ok(knowledge_base)
}

/// 返回某知识库所有的文件路径
pub async fn select_knowledge_base_files(
    db: &DbConn,
    name: String,
) -> Result<Vec<String>, sea_orm::DbErr> {
    let knowledge_base = knowledge_base::Entity::find()
        .filter(knowledge_base::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No Such Knowledge Base".into(),
        ))?;
    Ok(serde_json::from_value(knowledge_base.file_paths).unwrap_or_default())
}

/// 删除某个知识库
pub async fn delete_knowledge_base(db: &DbConn, name: String) -> Result<bool, sea_orm::DbErr> {
    let knowledge_base = knowledge_base::Entity::find()
        .filter(knowledge_base::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(sea_orm::DbErr::RecordNotFound(
            "No Such Knowledge Base".into(),
        ))?;
    let res = knowledge_base.delete(db).await?;
    Ok(res.rows_affected == 1)
}
