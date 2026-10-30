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
        .route("/disponiveis", get(listar).post(criar))
        .route("/disponiveis/:codigo", get(detalhar).put(editar).delete(excluir))
}

/// GET /v1/disponiveis — filtros: filtro, page, orderBy.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/disponiveis".to_string(), query, &headers, body).await
}

async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/disponiveis".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/disponiveis/{codigo}"), query, &headers, body).await
}

async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/disponiveis/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/disponiveis/{codigo}"), query, &headers, body).await
}
