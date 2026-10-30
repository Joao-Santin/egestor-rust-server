use axum::{
    body::Bytes, extract::{RawQuery, State}, http::HeaderMap, response::Response, routing::get, Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/empresa", get(detalhar))
}

/// GET /v1/empresa — dados cadastrais da empresa (razão social, CNPJ,
/// endereço etc).
async fn detalhar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/empresa".to_string(), query, &headers, body).await
}
