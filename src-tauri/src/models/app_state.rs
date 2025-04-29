use ollama_rust_api::Ollama;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub ollama: RwLock<Ollama>,
    pub db: RwLock<sea_orm::DbConn>,
    pub embedding_db: RwLock<Option<libsql::Connection>>,
}

impl AppState {
    pub async fn get_embedding_db(&self) -> Result<libsql::Connection, String> {
        let embedding_db = self.embedding_db.read().await;
        match &*embedding_db {
            Some(db) => Ok(db.clone()),
            _ => Err("Cannot connect to libsql db.".to_string()),
        }
    }
}
