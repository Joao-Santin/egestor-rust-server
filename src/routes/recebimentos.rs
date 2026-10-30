use axum::{
    body::Bytes,
    extract::{Path, RawQuery, State},
    http::HeaderMap,
    response::Response,
    routing::{get, put},
    Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/recebimentos", get(listar).post(criar))
        .route("/recebimentos/:codigo", get(detalhar).put(editar).delete(excluir))
        .route("/recebimentos/:codigo/receber", put(efetivar))
}

/// GET /v1/recebimentos — filtros: filtro, page, orderBy, updatedAfter,
/// updatedBefore, dtIni, dtFim, situacao (20=Em aberto, 40=Recebido).
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/recebimentos".to_string(), query, &headers, body).await
}

/// POST /v1/recebimentos — cria um recebimento (schema FinanceiroInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/recebimentos".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/recebimentos/{codigo}"), query, &headers, body).await
}

async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/recebimentos/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/recebimentos/{codigo}"), query, &headers, body).await
}

/// PUT /v1/recebimentos/{codigo}/receber — efetiva o recebimento
/// (schema EfetivarInput: valor, dtPgto). Retorna 403 se já estiver pago.
async fn efetivar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/recebimentos/{codigo}/receber"), query, &headers, body).await
}
