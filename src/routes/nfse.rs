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
        .route("/nfse", get(listar).post(emitir))
        .route("/nfse/:codigo", get(detalhar))
}

/// GET /v1/nfse — filtros: filtro, page, orderBy.
/// Requer ativação prévia do módulo NFSe com o suporte eGestor.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/nfse".to_string(), query, &headers, body).await
}

/// POST /v1/nfse — emite uma NFS-e (schema NFSeInput). Notas de serviço
/// não podem ser excluídas nem canceladas via API.
async fn emitir(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/nfse".to_string(), query, &headers, body).await
}

/// GET /v1/nfse/{codigo}
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfse/{codigo}"), query, &headers, body).await
}
