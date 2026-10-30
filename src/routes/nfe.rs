use axum::{
    body::Bytes,
    extract::{Path, RawQuery, State},
    http::HeaderMap,
    response::Response,
    routing::{get, post},
    Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/nfe", get(listar))
        .route("/nfe/salvar", post(salvar_xml))
        .route("/nfe/status", post(status_sefaz))
        .route("/nfe/recebidas", get(listar_recebidas))
        .route("/nfe/buscarNotas", get(buscar_notas))
        .route("/nfe/:codigo", get(detalhar))
        .route("/nfe/:codigo/xml", get(baixar_xml))
        .route("/nfe/:codigo/danfe", get(baixar_danfe))
        .route("/nfe/recebidas/:id/xml", get(recebida_xml))
        .route("/nfe/recebidas/:id/danfe", get(recebida_danfe))
        .route("/nfe/recebidas/:id/manifestar", post(manifestar))
        .route("/nfe/recebidas/:id/lancarCompra", post(lancar_compra))
}

/// GET /v1/nfe — **dtIni e dtFim são obrigatórios**. Filtros adicionais:
/// fields (aliases do schema NFe, ex.: "numero,chave,situacao,valor,dtEmissao"
/// — sem isso só `codigo` é retornado), filtro, page.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/nfe".to_string(), query, &headers, body).await
}

/// POST /v1/nfe/salvar — importa NF-e a partir de XML.
/// Corpo: { xml, codVenda?, enviar?, usarNumeracaoDoSistema? }.
async fn salvar_xml(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/nfe/salvar".to_string(), query, &headers, body).await
}

/// GET /v1/nfe/{codigo} — formato legado (schema NFeDetalhe).
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfe/{codigo}"), query, &headers, body).await
}

/// GET /v1/nfe/{codigo}/xml — retorna application/xml (só notas
/// autorizadas ou canceladas). Repassado como veio — Content-Type incluso.
async fn baixar_xml(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfe/{codigo}/xml"), query, &headers, body).await
}

/// GET /v1/nfe/{codigo}/danfe — retorna application/pdf.
async fn baixar_danfe(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfe/{codigo}/danfe"), query, &headers, body).await
}

/// POST /v1/nfe/status — consulta status do serviço SEFAZ.
async fn status_sefaz(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/nfe/status".to_string(), query, &headers, body).await
}

/// GET /v1/nfe/recebidas — **dtIni e dtFim são obrigatórios**.
/// Filtros adicionais: filtro, page.
async fn listar_recebidas(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/nfe/recebidas".to_string(), query, &headers, body).await
}

/// GET /v1/nfe/buscarNotas — consulta novas NF-es na SEFAZ (DFe).
/// Sujeito a cooldown (429 se consultado com muita frequência).
async fn buscar_notas(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/nfe/buscarNotas".to_string(), query, &headers, body).await
}

/// GET /v1/nfe/recebidas/{id}/xml — requer manifestação prévia (Ciência ou
/// Confirmação) para o XML estar disponível.
async fn recebida_xml(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfe/recebidas/{id}/xml"), query, &headers, body).await
}

/// GET /v1/nfe/recebidas/{id}/danfe
async fn recebida_danfe(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/nfe/recebidas/{id}/danfe"), query, &headers, body).await
}

/// POST /v1/nfe/recebidas/{id}/manifestar — corpo: { tpEvento, just? }.
/// tpEvento: 210210=Ciência, 210200=Confirmação, 210220=Desconhecimento,
/// 210240=Não realizada (requer `just` com >= 15 caracteres).
async fn manifestar(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/nfe/recebidas/{id}/manifestar"), query, &headers, body).await
}

/// POST /v1/nfe/recebidas/{id}/lancarCompra — corpo opcional:
/// { confirmarConsFinal? }.
async fn lancar_compra(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/nfe/recebidas/{id}/lancarCompra"), query, &headers, body).await
}
