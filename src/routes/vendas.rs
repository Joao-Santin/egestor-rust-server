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
        .route("/vendas", get(listar).post(criar))
        .route("/vendas/:codigo", get(detalhar).put(atualizar).delete(excluir))
        .route("/vendas/:codigo/contato", get(contato))
        .route("/vendas/:codigo/gerarNfse", get(gerar_nfse))
        .route("/vendas/:codigo/gerarNfseEnviarEmail", get(gerar_nfse_email))
        .route("/vendas/:codigo/gerarNfe", post(gerar_nfe))
        .route("/vendas/:codigo/gerarNfce", post(gerar_nfce))
        .route("/vendas/:codigo/nfe", get(nfes_da_venda))
        .route("/vendas/:codigo/mercadoLivre", get(mercado_livre))
        .route("/config/vendas/customizado", get(labels_customizado))
}

/// GET /v1/vendas — filtros: filtro, page, orderBy, updatedAfter,
/// updatedBefore, dtIni, dtFim, fields, dtTipo (dtVenda|dtCad|dtEntrega),
/// situacao (10=Orçamento, 50=Venda), codContato.
async fn listar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/vendas".to_string(), query, &headers, body).await
}

/// POST /v1/vendas — cria uma venda/orçamento (schema VendaInput).
async fn criar(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, "/v1/vendas".to_string(), query, &headers, body).await
}

/// GET /v1/vendas/{codigo} — filtro: listarCanceladas.
async fn detalhar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}"), query, &headers, body).await
}

/// PUT /v1/vendas/{codigo} — atualiza situação/campos customizados
/// (schema VendaUpdateInput). Para editar produtos/financeiros use os
/// endpoints de recebimentos/pagamentos ou recrie a venda.
async fn atualizar(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::PUT, format!("/v1/vendas/{codigo}"), query, &headers, body).await
}

async fn excluir(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::DELETE, format!("/v1/vendas/{codigo}"), query, &headers, body).await
}

/// GET /v1/vendas/{codigo}/contato
async fn contato(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}/contato"), query, &headers, body).await
}

/// GET /v1/vendas/{codigo}/gerarNfse — filtro: reterIssqn (bool).
async fn gerar_nfse(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}/gerarNfse"), query, &headers, body).await
}

/// GET /v1/vendas/{codigo}/gerarNfseEnviarEmail — filtro: reterIssqn (bool).
async fn gerar_nfse_email(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}/gerarNfseEnviarEmail"), query, &headers, body).await
}

/// POST /v1/vendas/{codigo}/gerarNfe — corpo opcional: { enviar, contigOffline }.
async fn gerar_nfe(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/vendas/{codigo}/gerarNfe"), query, &headers, body).await
}

/// POST /v1/vendas/{codigo}/gerarNfce — corpo opcional: { cpfcnpj, indPres, codTransportadora }.
async fn gerar_nfce(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/vendas/{codigo}/gerarNfce"), query, &headers, body).await
}

/// GET /v1/vendas/{codigo}/nfe — NF-es vinculadas à venda.
async fn nfes_da_venda(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}/nfe"), query, &headers, body).await
}

/// GET /v1/vendas/{codigo}/mercadoLivre
async fn mercado_livre(
    State(state): State<AppState>,
    Path(codigo): Path<i64>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, format!("/v1/vendas/{codigo}/mercadoLivre"), query, &headers, body).await
}

/// GET /v1/config/vendas/customizado — labels configurados para xCampo1..xCampo4.
async fn labels_customizado(
    State(state): State<AppState>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::GET, "/v1/config/vendas/customizado".to_string(), query, &headers, body).await
}
