use std::sync::Arc;

use crate::egestor_client::EgestorClient;

/// Estado compartilhado entre todas as rotas via `axum::extract::State`.
/// `Arc` torna o clone barato (só incrementa o contador de referências);
/// `EgestorClient` por dentro guarda o `reqwest::Client` (que já é
/// internamente compartilhável) e o `AuthManager` (protegido por um
/// `RwLock` assíncrono).
#[derive(Clone)]
pub struct AppState {
    pub egestor: Arc<EgestorClient>,
}
