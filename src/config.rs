/// Configuração do servidor, lida do ambiente (.env).
pub struct Config {
    pub port: u16,
    /// O `personal_token` (JWT) do eGestor — o que você cola em TOKENEGESTOR.
    pub personal_token: String,
    /// URL base da API (ex.: https://v4.egestor.com.br/api), sem barra final.
    pub base_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        // Carrega o .env se existir; se não existir, não é erro (ex.: variáveis
        // já exportadas no ambiente/container).
        let _ = dotenvy::dotenv();

        let personal_token = std::env::var("TOKENEGESTOR").unwrap_or_default();
        if personal_token.trim().is_empty() {
            return Err(
                "Faltou definir TOKENEGESTOR no seu .env. Copie .env.example para .env \
                 e preencha com o personal_token (JWT) gerado nas configurações da sua \
                 conta eGestor (Configurações → API)."
                    .to_string(),
            );
        }

        let port = std::env::var("PORT")
            .ok()
            .and_then(|v| v.parse::<u16>().ok())
            .unwrap_or(3000);

        let base_url = std::env::var("EGESTOR_BASE_URL")
            .unwrap_or_else(|_| "https://v4.egestor.com.br/api".to_string())
            .trim_end_matches('/')
            .to_string();

        Ok(Self { port, personal_token, base_url })
    }
}
