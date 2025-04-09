use ollama_rust_api::Ollama;
use sea_orm::DbConn;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct AppState {
    pub ollama: RwLock<Ollama>,
    pub db: RwLock<DbConn>,
}
