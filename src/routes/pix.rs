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
        .route("/pix", get(listar).post(criar))
        .route("/pix/credencial", get(credencial))
        .route("/pix/:codigo", get(detalhar))
        .route("/pix/:codigo/consultarSituacao", get(consultar_situacao))
}

/// GET /v1/pix — filtros: filtro, page.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/pix".to_string(), query, &headers, body).await
}

/// POST /v1/pix — cria cobrança PIX. Corpo: { codFinanceiro }.
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/pix".to_string(), query, &headers, body).await
}

/// GET /v1/pix/credencial — credenciais PIX configuradas na conta.
async fn credencial(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/pix/credencial".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/pix/{codigo}"), query, &headers, body).await
}

/// GET /v1/pix/{codigo}/consultarSituacao
async fn consultar_situacao(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/pix/{codigo}/consultarSituacao"), query, &headers, body).await
}
