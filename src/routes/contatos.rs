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
        .route("/contatos", get(listar).post(criar))
        .route("/contatos/:codigo", get(detalhar).put(editar).delete(excluir))
}

/// GET /v1/contatos — filtros conhecidos: filtro, page, orderBy,
/// updatedAfter, updatedBefore, fields, tipo (cliente|fornecedor|transportadora).
/// Qualquer outro filtro passado na query string também é repassado.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/contatos".to_string(), query, &headers, body).await
}

/// POST /v1/contatos — cria um contato (schema ContatoInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/contatos".to_string(), query, &headers, body).await
}

/// GET /v1/contatos/{codigo} — detalha um contato.
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/contatos/{codigo}"), query, &headers, body).await
}

/// PUT /v1/contatos/{codigo} — edita um contato (schema ContatoInput).
async fn editar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/contatos/{codigo}"), query, &headers, body).await
}

/// DELETE /v1/contatos/{codigo} — exclui um contato.
async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/contatos/{codigo}"), query, &headers, body).await
}
