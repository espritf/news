# server

Axum HTTP API (crate name `news`) backed by Postgres/pgvector: stores news items, chunks and
embeds their content via a local Ollama model, and serves semantic-similarity search over `GET
/news`.

## Project tree

```
server/
├── Cargo.toml             # Crate manifest (axum, diesel-async/postgres, pgvector, utoipa, ...)
├── Makefile                # DB/task shortcuts via psql+diesel (create/clean/drop/redo/show)
├── devenv.nix               # devenv shell: rust toolchain, postgres+pgvector, ollama process
├── devenv.lock               # Locked devenv/nix inputs
├── diesel.toml                # Diesel CLI config (schema output path, migrations dir, pgvector types)
├── .env                        # Local environment vars (DATABASE_URL, NEWS_API_TOKEN, OLLAMA_URL, ...)
├── .nvim.lua                    # Neovim project-local config (enables rust_analyzer)
├── migrations/                   # Diesel SQL migrations for the postgres schema
│   ├── .keep
│   ├── 00000000000000_diesel_initial_setup/           # Diesel's updated_at trigger helper functions
│   ├── 2023-12-11-100421_create_news/                 # Creates news table (title, pub_date, sources, title_v embedding)
│   ├── 2026-09-03-195350-0000_add_news_content/       # Adds news.content column
│   ├── 2026-09-04-140221-0000_create_news_chunks/     # Creates news_chunks table for per-chunk embeddings
│   ├── 2026-09-04-143114-0000_drop_news_title_v/      # Drops the now-unused news.title_v column
│   └── 2026-09-04-145346-0000_resize_chunk_v_to_1024/ # Resizes chunk_v to vector(1024) for the new embedding model
└── src/
    ├── main.rs             # Axum server entrypoint: loads env, builds AppState, wires routes/CORS/Swagger
    ├── app.rs              # AppState + NewsRepository/VectorProvider traits (mockable via automock)
    ├── pool.rs             # diesel-async deadpool Postgres connection pool
    ├── schema.rs           # Diesel-generated schema (news, news_chunks) — do not hand-edit
    ├── transfomer.rs       # Ollama embedding client implementing VectorProvider
    └── news/
        ├── mod.rs          # Re-exports the news submodules
        ├── handlers.rs     # HTTP handlers + OpenAPI/Swagger docs for GET/POST /news
        ├── model.rs        # News/NewsInput/NewsData/ChunkInput types, chunk_text splitting
        ├── lister.rs       # Lists news, embedding an optional search query for similarity ordering
        ├── publisher.rs    # Publishes a news item, embedding+storing chunks out-of-band
        ├── repository.rs   # NewsRepositoryImpl: diesel-async queries against Postgres/pgvector
        └── security.rs     # Auth middleware checking the `auth` header against the API token
```
