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
        .route("/discoVirtual", get(listar).post(enviar))
        .route("/discoVirtual/:codigo", get(detalhar).delete(excluir))
}

/// GET /v1/discoVirtual — filtros: filtro, page, orderBy.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/discoVirtual".to_string(), query, &headers, body).await
}

/// POST /v1/discoVirtual — multipart/form-data, campo `arquivo`. Assim
/// como o upload de foto de produto, o corpo é repassado em bytes brutos
/// com o Content-Type original (boundary incluso).
async fn enviar(
    State(state): State<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/discoVirtual".to_string(), None, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/discoVirtual/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/discoVirtual/{codigo}"), query, &headers, body).await
}
