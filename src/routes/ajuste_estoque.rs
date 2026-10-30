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
        .route("/ajusteEstoque", get(listar).post(criar))
        .route("/ajusteEstoque/:codigo", get(detalhar))
}

/// GET /v1/ajusteEstoque — filtros: filtro, page, orderBy.
/// Motivos: 10=Ajuste geral, 20=Envio RMA, 30=Retorno RMA,
/// 40=Devolução venda, 50=Devolução compra. Não editável/excluível via API.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/ajusteEstoque".to_string(), query, &headers, body).await
}

/// POST /v1/ajusteEstoque — schema AjusteEstoqueInput.
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/ajusteEstoque".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/ajusteEstoque/{codigo}"), query, &headers, body).await
}
