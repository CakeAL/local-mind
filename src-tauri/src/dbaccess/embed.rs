use std::path::Path;

use libsql::{Builder, Connection};

pub async fn get_embedding_db_conn(app_data_path: &Path) -> Result<Connection, libsql::Error> {
    let embedding_db_path = app_data_path.join("embedding.db");
    let embedding_db = Builder::new_local(embedding_db_path).build().await?;
    embedding_db.connect()
}

/// 用于创建新的知识库的时候存储向量的表
pub async fn new_embedding_table(
    embedding_db: &Connection,
    knowledge_base_name: &str,
) -> Result<(), libsql::Error> {
    let sql = format!(
        "CREATE TABLE IF NOT EXISTS {knowledge_base_name} (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT,
                embedding BLOB
            )"
    );
    embedding_db.execute(&sql, ()).await?;
    Ok(())
}

/// 插入向量到表中
pub async fn insert_embedding(
    embedding_db: &Connection,
    knowledge_base_name: &str,
    file_path: &str,
    embedding: &Vec<f32>,
) -> Result<u64, libsql::Error> {
    let embedding_str = format!("{:?}", embedding);
    let sql = format!(
        "INSERT INTO {knowledge_base_name} (file_path, embedding) VALUES (?1, vector32(?2))"
    );
    embedding_db
        .execute(&sql, (file_path, embedding_str.as_str()))
        .await
}

/// 删除某个文件的嵌入数据
pub async fn delete_embedding(
    embedding_db: &Connection,
    knowledge_base_name: &str,
    file_path: &str,
) -> Result<u64, libsql::Error> {
    let sql = format!("DELETE FROM {knowledge_base_name} WHERE file_path = ?1");
    // 修复：将参数包装为元组
    embedding_db.execute(&sql, libsql::params![file_path]).await
}

/// 删除某个知识库表
pub async fn delete_embedding_table(
    embedding_db: &Connection,
    knowledge_base_name: &str,
) -> Result<u64, libsql::Error> {
    let sql = format!("DROP TABLE IF EXISTS {knowledge_base_name}");
    embedding_db.execute(&sql, ()).await
}

#[cfg(test)]
mod tests {
    use ollama_rust_api::{model::embedding::EmbedRequestParameters, Ollama};

    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_get_embedding_db_conn() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        dbg!(embedding_db_conn);
    }

    #[tokio::test]
    async fn test_new_embedding_table() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        new_embedding_table(&embedding_db_conn, "测试")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_insert_embedding() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        let ollama = Ollama::default();
        let embed_response = ollama
            .embedding(&EmbedRequestParameters {
                model: "nomic-embed-text:137m-v1.5-fp16".into(),
                input: vec!["测试".into()],
            })
            .await
            .unwrap();
        for embedding in embed_response.embeddings.iter() {
            insert_embedding(&embedding_db_conn, "测试", "Test_Path", embedding)
                .await
                .unwrap();
        }
    }

    #[tokio::test]
    async fn test_delete_embedding() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        delete_embedding(&embedding_db_conn, "测试", "Test_Path")
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_delete_embedding_table() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        delete_embedding_table(&embedding_db_conn, "测试")
            .await
            .unwrap();
    }
}
