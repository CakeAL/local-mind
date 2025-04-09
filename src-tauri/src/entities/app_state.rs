use ollama_rust_api::Ollama;

#[derive(Default)]
pub struct AppState {
    ollama: Ollama,
}
