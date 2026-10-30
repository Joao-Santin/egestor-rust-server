use axum::{
    body::Bytes,
    extract::{Path, RawQuery, State},
    http::HeaderMap,
    response::Response,
    routing::post,
    Router,
};
use reqwest::Method;

use crate::{error::AppError, forward::forward, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/relatorios/:relatorio", post(gerar))
}

/// POST /v1/relatorios/{relatorio} — corpo típico: { de, ate, tipoData?, ordem? }
/// (varia por relatório; consulte os exemplos do spec para os parâmetros
/// específicos de cada um).
///
/// Identificadores válidos (`relatorio`), por categoria:
///
/// **Financeiro:** extratoFinanceiro, fluxoDeCaixa, fluxoFinanceiro, dre,
/// fluxoFuturo, fluxoFuturoDetalhado, lancamentosCancelados,
/// personalizadoFinanceiro, financeiroPorVendedor, comissoesFinanceiro,
/// custosDasCobrancas, conciliacaoBancaria, boletos
///
/// **Vendas:** vendasDetalhadas, detalhesProdutosVendidos, comissoesVendas,
/// abcProdutosVendidos, abcClientesVendas, vendasPorTipoDocumento,
/// vendasSemValores, vendasResumo
///
/// **Compras:** comprasDetalhadas, detalhesProdutosComprados,
/// producoesDetalhadas, composicoes
///
/// **Contatos:** historicoContato, personalizadoContatos,
/// aniversariantesDoMes, inatividadeClientes, contatosDuplicados
///
/// **Estoque/Produtos:** produtosPorFornecedor, estoqueMinimo,
/// estoqueDoDia, estoqueHistorico, sugestaoCompra, produtosDuplicados,
/// movimentacaoProduto, personalizadoProdutos
///
/// **Fiscal:** nfseImpostos, nfeResumo, nfeTotaisPorPeriodo
///
/// **Sistema:** registroEventos, registroAcessos
///
/// Exemplo: POST /api/relatorios/vendasResumo  { "de": "2025-01-01", "ate": "2025-01-31" }
async fn gerar(
    State(state): State<AppState>,
    Path(relatorio): Path<String>,
    RawQuery(query): RawQuery,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    forward(&state, Method::POST, format!("/v1/relatorios/{relatorio}"), query, &headers, body).await
}
