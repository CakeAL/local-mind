use sea_orm::{ConnectionTrait, DbBackend, DbConn, Schema, Statement};

use crate::entities::conversation;

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
        tracing::info!("Table conversation has created")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::dbaccess::{conversation::create_conversation_table, get_db_conn};

    #[tokio::test]
    async fn test_create_conversation_table() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let db_conn = get_db_conn(&app_data_path).await.unwrap();
        let res = create_conversation_table(&db_conn).await;
        dbg!(res.unwrap());
    }
}
