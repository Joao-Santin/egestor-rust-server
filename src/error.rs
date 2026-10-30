use axum::{
    body::Body,
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
};

/// Erros do PRÓPRIO gateway (não confundir com respostas de erro que o
/// eGestor devolve — essas são simplesmente repassadas como vieram, com o
/// mesmo status e corpo, em `egestor_client::into_axum_response`).
#[derive(Debug)]
pub enum AppError {
    /// Falha de rede/protocolo ao tentar falar com a API do eGestor
    /// (timeout, DNS, TLS, conexão recusada etc).
    Upstream(reqwest::Error),
    /// Qualquer outro erro interno (autenticação mal configurada, falha ao
    /// montar a resposta etc).
    Internal(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Upstream(e) => write!(f, "erro ao comunicar com a API do eGestor: {e}"),
            AppError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        tracing::error!("{self}");

        let (status, msg) = match &self {
            AppError::Upstream(_) => (
                StatusCode::BAD_GATEWAY,
                "Não foi possível se comunicar com a API do eGestor.".to_string(),
            ),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = serde_json::json!({ "erro": msg }).to_string();

        Response::builder()
            .status(status)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
    }
}
