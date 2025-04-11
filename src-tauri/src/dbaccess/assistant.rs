use sea_orm::{ConnectionTrait, DbConn, Schema, Statement};

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
