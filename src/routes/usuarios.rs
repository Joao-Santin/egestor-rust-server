use axum::{
    body::Bytes,
    extract::{Path, RawQuery, State},
    http::HeaderMap,
    response::Response,
    routing::get,
    Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/usuarios", get(listar))
        .route("/usuarios/:user_id", get(detalhar))
}

/// GET /v1/usuarios — filtros: filtro, page, orderBy, fields, vendedor
/// (bool), usuarioSistema (bool).
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/usuarios".to_string(), query, &headers, body).await
}

/// GET /v1/usuarios/{userId}
async fn detalhar(
    State(state): State<AppState>,
    Path(user_id): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/usuarios/{user_id}"), query, &headers, body).await
}
