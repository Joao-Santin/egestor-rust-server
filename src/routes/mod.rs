mod ajuste_estoque;
mod boletos;
mod categorias;
mod compras;
mod config_trib;
mod contatos;
mod devolucoes;
mod disco_virtual;
mod disponiveis;
mod empresa;
mod formas_pagamento;
mod health;
mod nfe;
mod nfse;
mod pagamentos;
mod pix;
mod plano_contas;
mod produtos;
mod producao;
mod recebimentos;
mod relatorios;
mod servicos;
mod usuarios;
mod vendas;
mod webhooks;

use axum::Router;

use crate::state::AppState;

/// Monta o Router final: tudo sob `/api/*` (espelhando `/v1/*` e `/oauth/*`
/// da API do eGestor) mais `/health` na raiz.
pub fn build(state: AppState) -> Router {
    let api = Router::new()
        .merge(contatos::router())
        .merge(produtos::router())
        .merge(servicos::router())
        .merge(vendas::router())
        .merge(recebimentos::router())
        .merge(pagamentos::router())
        .merge(boletos::router())
        .merge(nfse::router())
        .merge(nfe::router())
        .merge(pix::router())
        .merge(compras::router())
        .merge(producao::router())
        .merge(categorias::router())
        .merge(usuarios::router())
        .merge(disponiveis::router())
        .merge(plano_contas::router())
        .merge(formas_pagamento::router())
        .merge(ajuste_estoque::router())
        .merge(disco_virtual::router())
        .merge(config_trib::router())
        .merge(devolucoes::router())
        .merge(webhooks::router())
        .merge(relatorios::router())
        .merge(empresa::router());

    Router::new()
        .nest("/api", api)
        .merge(health::router())
        .with_state(state)
}
