# egestor-api-server (Rust)

Gateway em Rust (`tokio` + `axum` + `reqwest` + `serde`) para a API REST do
eGestor (https://v4.egestor.com.br/www/api/docs/), com autenticação OAuth2
automática a partir de um `personal_token`.

## ⚠️ Aviso importante

Este projeto foi escrito num ambiente **sem acesso à internet e sem
toolchain Rust instalada**, então eu não consegui rodar `cargo build` para
validar a compilação. Revisei o código com bastante cuidado (tipos,
extractors do axum 0.7, assinaturas do reqwest 0.12), mas é possível que
sobre algum erro pontual de compilação na sua máquina. **Se isso acontecer,
me cole o erro do `cargo build` aqui no chat que eu corrijo na hora.**

## Como funciona a autenticação

O spec (`egestor_openapi.yml`) documenta OAuth2 assim:

1. Você gera um **`personal_token`** (JWT de longa duração) no painel do
   eGestor, em Configurações → API.
2. Esse `personal_token` é trocado por um **`access_token`**, válido por
   apenas **15 minutos**, via `POST /oauth/access_token`.
3. Esse `access_token` é o que vai no header `Authorization: Bearer ...`
   de toda chamada.

Você só precisa colar o `personal_token` em `TOKENEGESTOR` no `.env` — o
servidor troca por `access_token` sozinho e renova automaticamente antes de
expirar (com margem de segurança de 30s), e também reautentica sozinho se o
eGestor devolver 401 no meio do caminho.

## Design: rotas dedicadas, corpo/resposta em passthrough

Cada recurso do spec (contatos, produtos, vendas, produção, financeiro,
fiscal etc.) tem seu próprio arquivo de rotas em `src/routes/`, com uma
rota Rust dedicada para cada endpoint documentado. Em vez de tentar recriar
em Rust as dezenas de schemas de request/response do spec (arriscado — um
campo digitado errado quebra silenciosamente), cada rota:

- aceita **qualquer filtro** que você mandar na query string e repassa
  exatamente como veio para o eGestor (os filtros oficiais de cada endpoint
  estão documentados no comentário `///` acima de cada handler, mas
  qualquer outro filtro que você "pensar" também passa, sem precisar mexer
  no código);
- repassa o corpo da requisição (JSON ou multipart/form-data, com o
  Content-Type original) sem tentar decodificá-lo — por isso upload de foto
  de produto e de disco virtual já funcionam sem código especial;
- devolve a resposta do eGestor (status, Content-Type e corpo) exatamente
  como veio — inclusive respostas binárias (XML de NF-e, PDF de DANFE) e
  erros (400/401/404/410/422/429, já no formato `ErrorResponse` do eGestor).

Isso significa que o gateway nunca fica desatualizado em relação ao schema
real da API. Se quiser validação forte de tipos em algum endpoint
específico (ex.: `VendaInput` com campos obrigatórios), é só me pedir que eu
adiciono um `#[derive(Deserialize)]` para aquele recurso.

## Instalação

Requer Rust 1.75+ (edition 2021). Se não tiver o toolchain instalado:
https://rustup.rs

```bash
cp .env.example .env
```

Edite o `.env` e preencha:

```
TOKENEGESTOR=seu_personal_token_aqui
```

## Rodando

```bash
cargo run
```

Em desenvolvimento, para recompilar automaticamente a cada mudança, instale
o `cargo-watch` (`cargo install cargo-watch`) e rode `cargo watch -x run`.

O servidor sobe em `http://localhost:3000` (ou na `PORT` que você definir).

## Estrutura

```
egestor-rust-server/
├── Cargo.toml
├── .env.example
└── src/
    ├── main.rs              # ponto de entrada (axum::serve)
    ├── config.rs            # leitura/validação do .env
    ├── state.rs             # AppState compartilhado (Arc<EgestorClient>)
    ├── error.rs             # AppError -> resposta HTTP
    ├── auth.rs              # OAuth2: personal_token -> access_token (cache + renovação)
    ├── egestor_client.rs    # cliente HTTP: injeta auth, repassa corpo/headers/status
    ├── forward.rs           # helper usado por todas as rotas
    └── routes/
        ├── mod.rs           # agrega todos os routers sob /api
        ├── health.rs        # GET /health
        ├── contatos.rs
        ├── produtos.rs
        ├── servicos.rs
        ├── vendas.rs        # + gerarNfse/gerarNfe/gerarNfce, contato, Mercado Livre
        ├── recebimentos.rs
        ├── pagamentos.rs
        ├── boletos.rs
        ├── nfse.rs
        ├── nfe.rs           # emitidas + recebidas (12 endpoints)
        ├── pix.rs
        ├── compras.rs
        ├── producao.rs      # 👈 o recurso do pedido original
        ├── categorias.rs
        ├── usuarios.rs
        ├── disponiveis.rs
        ├── plano_contas.rs
        ├── formas_pagamento.rs
        ├── ajuste_estoque.rs
        ├── disco_virtual.rs
        ├── config_trib.rs
        ├── devolucoes.rs
        ├── webhooks.rs
        ├── relatorios.rs    # POST /api/relatorios/:relatorio (~40 relatórios)
        └── empresa.rs
```

## Exemplos de uso

```bash
# Health check
curl http://localhost:3000/health

# Produções filtrando por período e situação (qualquer filtro do spec funciona)
curl "http://localhost:3000/api/producao?dtIni=2025-01-01&dtFim=2025-01-31&situacao=50"

# Detalhar uma produção
curl "http://localhost:3000/api/producao/123"

# Criar uma produção
curl -X POST http://localhost:3000/api/producao \
  -H "Content-Type: application/json" \
  -d '{"produtos":[{"codProduto":1,"quantProd":10}]}'

# Vendas de um contato num período
curl "http://localhost:3000/api/vendas?codContato=1&dtIni=2025-01-01&dtFim=2025-01-31&dtTipo=dtVenda"

# Gerar relatório de resumo de vendas
curl -X POST http://localhost:3000/api/relatorios/vendasResumo \
  -H "Content-Type: application/json" \
  -d '{"de":"2025-01-01","ate":"2025-01-31"}'

# Baixar o XML de uma NF-e (retorna o arquivo binário)
curl "http://localhost:3000/api/nfe/123/xml" -o nota.xml

# Upload de foto de produto (multipart, repassado como veio)
curl -X POST http://localhost:3000/api/produtos/123/fotos \
  -F "foto=@/caminho/para/imagem.jpg"
```

Todas as rotas seguem o padrão `/api/{recurso}` espelhando `/v1/{recurso}`
do eGestor — dá pra conferir o mapeamento completo em `src/routes/mod.rs`
e nos comentários de cada handler.

## Rate limiting

O eGestor limita a 60 requisições/minuto por aplicação+usuário e devolve
`429` quando estourado (junto com os headers `X-RateLimit-Limit` e
`X-RateLimit-Remaining`, que este gateway repassa). Não implementei
throttling do lado do gateway — se isso for útil pra você, posso adicionar
uma fila/limitador (ex.: crate `governor`).

## Próximos passos possíveis

- Adicionar structs `Deserialize`/`Serialize` tipadas para recursos
  específicos que você mais usa (troca a validação "o eGestor que valide"
  por validação já no gateway, com erro 422 antes mesmo de sair pra rede).
- Rate limiting local.
- Testes de integração com um mock do eGestor (`wiremock`).
