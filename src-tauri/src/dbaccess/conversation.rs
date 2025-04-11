use sea_orm::{ConnectionTrait, DbBackend, DbConn, Schema, Statement};

use crate::entities::conversation;

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

// / 插入新的一段消息
// pub async fn insert_message(db: &DbConn) -> Result<(), sea_orm::DbErr> {

//     Ok(())
// }
