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
        .route("/boletos", get(listar).post(criar))
        .route("/boletos/:codigo", get(detalhar))
        .route("/verificarRetornoBoletos", get(verificar_retorno))
}

/// GET /v1/boletos — filtros: filtro, page, orderBy.
/// Situações: 10=Em aberto, 50=Recebido. Boletos não podem ser editados
/// nem excluídos via API.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/boletos".to_string(), query, &headers, body).await
}

/// POST /v1/boletos — cria um boleto (schema BoletoInput: codContato,
/// dtVenc, codRecebimentos[]).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/boletos".to_string(), query, &headers, body).await
}

/// GET /v1/boletos/{codigo} — filtro: mostrarInativos (bool).
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/boletos/{codigo}"), query, &headers, body).await
}

/// GET /v1/verificarRetornoBoletos — processa o retorno bancário e
/// atualiza os boletos pagos.
async fn verificar_retorno(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/verificarRetornoBoletos".to_string(), query, &headers, body).await
}
