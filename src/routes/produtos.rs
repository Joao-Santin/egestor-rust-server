use axum::{
    body::Bytes,
    extract::{Path, RawQuery, State},
    http::HeaderMap,
    response::Response,
    routing::{get, post},
    Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/produtos", get(listar).post(criar))
        .route("/produtos/:codigo", get(detalhar).put(editar).delete(excluir))
        .route("/produtos/:codigo/fotos", post(enviar_foto))
}

/// GET /v1/produtos — filtros: filtro, page, orderBy, updatedAfter,
/// updatedBefore, fields.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/produtos".to_string(), query, &headers, body).await
}

/// POST /v1/produtos — cria um produto (schema ProdutoInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/produtos".to_string(), query, &headers, body).await
}

/// GET /v1/produtos/{codigo}
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/produtos/{codigo}"), query, &headers, body).await
}

/// PUT /v1/produtos/{codigo}
async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/produtos/{codigo}"), query, &headers, body).await
}

/// DELETE /v1/produtos/{codigo}
async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/produtos/{codigo}"), query, &headers, body).await
}

/// POST /v1/produtos/{codigo}/fotos — multipart/form-data, campo `foto`
/// (máx. 10 MB, conforme spec). O corpo multipart (com seu boundary) é
/// repassado em bytes brutos, junto com o Content-Type original — não há
/// necessidade de reconstruir o multipart aqui.
async fn enviar_foto(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/produtos/{codigo}/fotos"), None, &headers, body).await
}
