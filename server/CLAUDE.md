# server

Axum HTTP API (crate name `news`) backed by Postgres/pgvector: stores news items, chunks and
embeds their content via a local Ollama model, and serves semantic-similarity search over `GET
/news`. Embedding runs in a separate worker binary (`cargo run --bin embedder`) that drains a
Postgres-backed `embedding_jobs` queue.

## Project tree

```
server/
├── Cargo.toml             # Crate manifest (axum, diesel-async/postgres, pgvector, utoipa, ...)
├── Makefile                # DB/task shortcuts via psql+diesel (create/clean/drop/redo/show)
├── devenv.nix               # devenv shell: rust toolchain, postgres+pgvector, clickhouse, ollama process
├── devenv.lock               # Locked devenv/nix inputs
├── diesel.toml                # Diesel CLI config (schema output path, migrations dir, pgvector types)
├── .env                        # Local environment vars (DATABASE_URL, NEWS_API_TOKEN, OLLAMA_URL, ...)
├── .nvim.lua                    # Neovim project-local config (enables rust_analyzer)
├── clickhouse/
│   └── events.sql                # ClickHouse statistics schema (embedding_events); applied by `make clickhouse`
├── migrations/                   # Diesel SQL migrations for the postgres schema
│   ├── .keep
│   ├── 00000000000000_diesel_initial_setup/           # Diesel's updated_at trigger helper functions
│   ├── 2023-12-11-100421_create_news/                 # Creates news table (title, pub_date, sources, title_v embedding)
│   ├── 2026-09-03-195350-0000_add_news_content/       # Adds news.content column
│   ├── 2026-09-04-140221-0000_create_news_chunks/     # Creates news_chunks table for per-chunk embeddings
│   ├── 2026-09-04-143114-0000_drop_news_title_v/      # Drops the now-unused news.title_v column
│   ├── 2026-09-04-145346-0000_resize_chunk_v_to_1024/ # Resizes chunk_v to vector(1024) for the new embedding model
│   └── 2026-10-15-120000-0000_create_embedding_jobs/  # Creates embedding_jobs queue (retries, leases) + backfill
└── src/
    ├── lib.rs              # Library root shared by both binaries
    ├── main.rs             # Axum server entrypoint: loads env, builds AppState, wires routes/CORS/Swagger
    ├── app.rs              # AppState + NewsRepository/EmbeddingJobRepository/EventSink/VectorProvider traits (automock)
    ├── events.rs           # EmbeddingEvent + ClickHouseEvents (EventSink impl) for statistics
    ├── pool.rs             # diesel-async deadpool Postgres connection pool
    ├── schema.rs           # Diesel-generated schema (news, news_chunks) — do not hand-edit
    ├── transfomer.rs       # Ollama embedding client implementing VectorProvider
    ├── bin/
    │   └── embedder.rs     # Embedding worker entrypoint: drains embedding_jobs
    ├── embedder/
    │   ├── mod.rs          # Re-exports the embedder submodules
    │   ├── model.rs        # EmbeddingJob (a leased queue row + article content)
    │   ├── repository.rs   # EmbeddingJobRepositoryImpl: claim (SKIP LOCKED lease), complete, fail
    │   └── worker.rs       # EmbeddingWorker: chunk, embed, store; exponential backoff on failure
    └── news/
        ├── mod.rs          # Re-exports the news submodules
        ├── chunker.rs      # chunk_text: splits text into word-bounded chunks for embedding
        ├── handlers.rs     # HTTP handlers + OpenAPI/Swagger docs for GET/POST /news
        ├── model.rs        # News/NewsInput/NewsData/ChunkInput types
        ├── lister.rs       # Lists news, embedding an optional search query for similarity ordering
        ├── publisher.rs    # Publishes a news item + its embedding job in one transaction
        ├── repository.rs   # NewsRepositoryImpl: diesel-async queries against Postgres/pgvector
        └── security.rs     # Auth middleware checking the `auth` header against the API token
```
