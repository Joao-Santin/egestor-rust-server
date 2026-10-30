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
        .route("/compras", get(listar).post(criar))
        .route("/compras/:codigo", get(detalhar).delete(excluir))
        .route("/compras/:codigo/efetivar", get(efetivar))
        .route("/compras/:codigo/reabrir", get(reabrir))
        .route("/compras/:codigo/nfe", get(nfes_da_compra))
}

/// GET /v1/compras — filtros: filtro, page, orderBy, dtIni, dtFim, fields,
/// dtTipo (dtCompra|dtCad), situacao (10=Orçamento, 50=Compra efetivada),
/// nota, observacoes.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/compras".to_string(), query, &headers, body).await
}

/// POST /v1/compras — cria compra (schema CompraInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/compras".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/compras/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/compras/{codigo}"), query, &headers, body).await
}

/// GET /v1/compras/{codigo}/efetivar — filtro: atualizarPrecoVenda (bool).
async fn efetivar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/compras/{codigo}/efetivar"), query, &headers, body).await
}

/// GET /v1/compras/{codigo}/reabrir
async fn reabrir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/compras/{codigo}/reabrir"), query, &headers, body).await
}

/// GET /v1/compras/{codigo}/nfe
async fn nfes_da_compra(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/compras/{codigo}/nfe"), query, &headers, body).await
}
