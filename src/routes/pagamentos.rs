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
        .route("/pagamentos", get(listar).post(criar))
        .route("/pagamentos/:codigo", get(detalhar).put(editar).delete(excluir))
        .route("/pagamentos/:codigo/pagar", put(efetivar))
        .route("/pagamentos/:codigo/contato", get(contato))
}

/// GET /v1/pagamentos — filtros: filtro, page, orderBy, updatedAfter,
/// updatedBefore, dtIni, dtFim, situacao (10=Em aberto, 30=Pago).
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/pagamentos".to_string(), query, &headers, body).await
}

/// POST /v1/pagamentos — cria um pagamento (schema FinanceiroInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/pagamentos".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/pagamentos/{codigo}"), query, &headers, body).await
}

async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/pagamentos/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/pagamentos/{codigo}"), query, &headers, body).await
}

/// PUT /v1/pagamentos/{codigo}/pagar — efetiva o pagamento
/// (schema EfetivarInput: valor, dtPgto). Retorna 403 se já estiver pago.
async fn efetivar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/pagamentos/{codigo}/pagar"), query, &headers, body).await
}

/// GET /v1/pagamentos/{codigo}/contato
async fn contato(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/pagamentos/{codigo}/contato"), query, &headers, body).await
}
