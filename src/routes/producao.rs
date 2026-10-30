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
        .route("/producao", get(listar).post(criar))
        .route("/producao/:codigo", get(detalhar).put(editar).delete(excluir))
        .route("/producao/:codigo/reabrir", get(reabrir))
}

/// GET /v1/producao — filtros: filtro, page, orderBy, dtIni, dtFim, fields,
/// situacao (10=Aberta, 50=Concluída). Qualquer outro filtro na query
/// string também é repassado ao eGestor.
///
/// Exemplo: GET /api/producao?dtIni=2025-01-01&dtFim=2025-01-31&situacao=50
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/producao".to_string(), query, &headers, body).await
}

/// POST /v1/producao — cria uma ordem de produção (schema ProducaoInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/producao".to_string(), query, &headers, body).await
}

/// GET /v1/producao/{codigo} — dados completos (produtos + pagamentos).
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/producao/{codigo}"), query, &headers, body).await
}

/// PUT /v1/producao/{codigo} — edita (schema ProducaoInput). `produtos` é
/// sempre reenviado por completo; `pagamentos` é incremental por `codigo`.
/// Não reabre uma produção concluída — use GET .../reabrir para isso.
async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/producao/{codigo}"), query, &headers, body).await
}

/// DELETE /v1/producao/{codigo} — não permite excluir produções concluídas
/// nem com pagamentos lançados.
async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/producao/{codigo}"), query, &headers, body).await
}

/// GET /v1/producao/{codigo}/reabrir — reverte uma produção concluída
/// (situação 50) de volta para aberta (situação 10).
async fn reabrir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/producao/{codigo}/reabrir"), query, &headers, body).await
}
