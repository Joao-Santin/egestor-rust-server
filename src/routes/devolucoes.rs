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
        .route("/devolucoes", get(listar).post(criar))
        .route("/devolucoes/:codigo", get(detalhar))
}

/// GET /v1/devolucoes — filtros: filtro, page, orderBy, fields.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/devolucoes".to_string(), query, &headers, body).await
}

/// POST /v1/devolucoes — schema DevolucaoInput. Permite editar o
/// recebimento da venda (finReceber), gerar pagamento ao cliente
/// (finPagar), ou apenas ajustar estoque (omitindo ambos).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/devolucoes".to_string(), query, &headers, body).await
}

async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/devolucoes/{codigo}"), query, &headers, body).await
}
