use axum::{
    body::Bytes, extract::{RawQuery, State}, http::HeaderMap, response::Response, routing::get, Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/webhooks", get(listar).post(cadastrar))
}

/// GET /v1/webhooks — lista webhooks cadastrados.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/webhooks".to_string(), query, &headers, body).await
}

/// POST /v1/webhooks — schema WebhookInput: { url, evento }.
async fn cadastrar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/webhooks".to_string(), query, &headers, body).await
}
