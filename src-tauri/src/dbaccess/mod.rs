pub mod assistant;
pub mod conversation;

use std::path::Path;

use sea_orm::{Database, DbConn};

pub async fn get_db_conn(app_data_path: &Path) -> Result<DbConn, sea_orm::DbErr> {
    let db_path = app_data_path.join("data.db");
    let db_path = db_path.to_string_lossy();
    // 如果没有就创建
    let db = Database::connect(format!("sqlite://{}?mode=rwc", db_path)).await?;
    tracing::info!("Connected to sqlite. db_path = {}", &db_path);
    // 创建 tables
    assistant::create_assistant_table(&db).await?;
    conversation::create_conversation_table(&db).await?;
    Ok(db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_create_tables() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let db_conn = get_db_conn(&app_data_path).await.unwrap();
        dbg!(db_conn);
    }
}
