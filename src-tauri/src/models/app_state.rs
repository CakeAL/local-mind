use ollama_rust_api::Ollama;
use tokio::sync::RwLock;

pub struct AppState {
    pub ollama: RwLock<Ollama>,
    pub db: RwLock<sea_orm::DbConn>,
    pub embedding_db: RwLock<libsql::Connection>,
}
