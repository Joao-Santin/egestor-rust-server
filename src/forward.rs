use axum::{body::Bytes, http::HeaderMap, response::Response};
use reqwest::Method;

use crate::{error::AppError, state::AppState};

/// Usado por todos os módulos em `src/routes/*`: encaminha a requisição
/// recebida para o path correspondente na API do eGestor.
pub async fn forward(
    state: &AppState,
    method: Method,
    upstream_path: String,
    raw_query: Option<String>,
    headers: &HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    state
        .egestor
        .proxy(method, &upstream_path, raw_query.as_deref(), headers, body)
        .await
}
