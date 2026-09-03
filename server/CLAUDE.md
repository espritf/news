# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

An Axum API server backed by Postgres + [pgvector](https://github.com/pgvector/pgvector): it
accepts published news items (token-authenticated), generates a title embedding via a local
[Ollama](https://ollama.ai) model, stores them, and serves a news listing with optional semantic
search.

```
POST /news ──▶  embeds title via Ollama  ──▶  Postgres + pgvector  ◀──  GET /news (+ search)
```

## Commands

```sh
devenv shell             # provides diesel-cli, openssl, pkg-config, ollama, rust-analyzer,
                          # and starts Postgres with pgvector automatically (services.postgres)
make create               # run diesel migrations (needs diesel-cli, DATABASE_URL)
ollama pull <model>        # pull the model configured as EMBEDDING_MODEL, e.g. nomic-embed-text
cargo run
cargo check
cargo clippy
cargo test                 # unit tests live inline in src/news/handlers.rs
cargo test test_create_auth  # run a single test
```

Other Makefile targets: `make clean` (delete all rows from `news`), `make drop` (drop the
Postgres database), `make redo` (redo the last migration).

### Environment

Config is loaded via `dotenv-flow` (`.env` then `.env.local` overrides), see `src/main.rs`.

- `.env` — shared, checked in: `DATABASE_URL`, `POSTGRES_*`, `SERVER_ADDR`, `EMBEDDING_MODEL`, `RUST_LOG`
- `.env.local` — gitignored, overrides locally: `NEWS_API_TOKEN`, `RUST_LOG`

`EMBEDDING_MODEL` must match a model already pulled into the local Ollama daemon (see
`Modelfile` for an example custom model); its output vector dimension must match the
`title_v vector(N)` column in `migrations/`.

## Architecture

### Request flow (`src/news/handlers.rs`)

`news::handlers::routes(token)` builds the router: `POST /news` is wrapped in
`middleware::from_fn_with_state(token, security::auth)` (checks an `auth` header against
`NEWS_API_TOKEN`), `GET /news` is unauthenticated. Both handlers pull
`AppState` out of Axum state and go through `AppError` (`impl IntoResponse` + blanket `From<E:
Into<anyhow::Error>>`) so handler bodies can just use `?`.

- `publish` embeds the title via `state.model.vector(title)`, builds `NewsData` (input +
  embedding), and calls `state.repo.create`.
- `list` embeds the optional `search` query string the same way, wraps it in `ListParams`, and
  calls `state.repo.list`. No `search` means a plain paginated listing.

### `AppState` / trait seams (`src/app.rs`)

`AppState` holds `Arc<dyn NewsRepository>` and `Arc<dyn VectorProvider>` — the seam between HTTP
handling and both persistence and embedding, and what `#[cfg_attr(test, automock)]` (via
`mockall`) mocks in `src/news/handlers.rs` tests. Anything reachable through these traits can be
swapped without touching handlers — e.g. `transfomer::ollama::Model` is the only
`VectorProvider` impl today, hitting `http://localhost:11434/api/embeddings` directly (not
configurable — same host is assumed).

### Persistence (`src/news/repository.rs`, `src/pool.rs`, Diesel + `diesel-async`)

`NewsRepositoryImpl` wraps a `pool::Pool` (a `deadpool`-backed `diesel-async` pool). `list`
builds its `ORDER BY` dynamically: with a search vector it orders by `title_v.l2_distance(query)
ASC` (nearest-neighbor search via pgvector), otherwise by `pub_date DESC` — both branches are
boxed into the same `BoxableExpression` type so the query builder stays uniform. `create` is a
plain `insert_into(news::table).returning(...)`.

Schema is Diesel-generated (`diesel.toml` → `src/schema.rs`); after adding a migration under
`migrations/`, run `make create` (or `diesel migration run`) to apply it and regenerate the
schema file — don't hand-edit `schema.rs`. The single `news` table stores `sources` as a JSON
array of source URLs, plus `title_v` (pgvector `vector(768)`, dimension fixed by the embedding
model in use).

### Error handling

`anyhow::Result` throughout; `AppError` (`src/news/handlers.rs`) is the only place errors surface
as HTTP — every error becomes a bare `500` (message logged via `tracing::error!`, not returned to
the client). There's no per-error-kind status mapping (e.g. a bad `search` embedding and a DB
outage look identical to the caller).
