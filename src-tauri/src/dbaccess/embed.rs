use std::path::Path;

use libsql::{Builder, Connection};
use serde::Serialize;

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
                content TEXT,
                file_path TEXT,
                embedding BLOB
            )"
    );
    embedding_db.execute(&sql, ()).await?;
    // 创建向量索引，用于提升速度
    let sql = format!(
        "CREATE INDEX {knowledge_base_name}_idx ON {knowledge_base_name} (libsql_vector_idx(embedding));"
    );
    embedding_db.execute(&sql, ()).await?;
    Ok(())
}

/// 插入向量到表中
pub async fn insert_embedding(
    embedding_db: &Connection,
    knowledge_base_name: &str,
    content: &str,
    file_path: &str,
    embedding: &Vec<f32>,
) -> Result<u64, libsql::Error> {
    let embedding_str = format!("{:?}", embedding);
    let sql = format!(
        "INSERT INTO {knowledge_base_name} (content, file_path, embedding) VALUES (?1, ?2, vector32(?3))"
    );
    embedding_db
        .execute(&sql, (content, file_path, embedding_str.as_str()))
        .await
}

/// 删除某个文件的嵌入数据
pub async fn delete_embedding(
    embedding_db: &Connection,
    knowledge_base_name: &str,
    file_path: &str,
) -> Result<u64, libsql::Error> {
    let sql = format!("DELETE FROM {knowledge_base_name} WHERE file_path = ?1");
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

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub content: String,
    pub file_path: String,
    pub distance: f64,
}

/// 向量相似度搜索
pub async fn search_similar_embeddings(
    embedding_db: &Connection,
    knowledge_base_name: &str,
    query_embedding: &Vec<f32>,
) -> Result<Vec<SearchResult>, libsql::Error> {
    let query_embedding_str = format!("{:?}", query_embedding);
    let sql = format!(
        "SELECT content, file_path, vector_extract(embedding),
            vector_distance_cos(embedding, vector32(?1))
        FROM {knowledge_base_name}
        WHERE
            vector_distance_cos(embedding, vector32(?1)) < 0.5
        ORDER BY
            vector_distance_cos(embedding, vector32(?1))
        ASC"
    );
    let mut rows = embedding_db
        .query(&sql, libsql::params![query_embedding_str.as_str()])
        .await?;
    let mut results = Vec::new();
    while let Some(row) = rows.next().await? {
        let content: String = row.get(0)?;
        let file_path: String = row.get(1)?;
        let distance = row.get::<f64>(3)?;
        results.push(SearchResult {
            content,
            file_path,
            distance,
        });
    }
    Ok(results)
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
                model: "bge-m3:567m".into(),
                input: vec!["这是一段测试文字".into()],
            })
            .await
            .unwrap();
        for embedding in embed_response.embeddings.iter() {
            insert_embedding(
                &embedding_db_conn,
                "测试",
                "这是一段测试文字",
                "Test_Path",
                embedding,
            )
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

    #[tokio::test]
    async fn test_search_similar_embeddings() {
        let app_data_path =
            PathBuf::from("/Users/cakeal/Library/Application Support/com.local-mind/");
        let embedding_db_conn = get_embedding_db_conn(&app_data_path).await.unwrap();
        let ollama = Ollama::default();
        let embed_response = ollama
            .embedding(&EmbedRequestParameters {
                model: "bge-m3:567m".into(),
                input: vec!["测试文字".into()],
            })
            .await
            .unwrap();
        let res =
            search_similar_embeddings(&embedding_db_conn, "测试", &embed_response.embeddings[0])
                .await
                .unwrap();
        dbg!(res);
    }
}
