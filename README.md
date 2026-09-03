# news

A small pipeline that collects news from RSS/HTML sources, translates and publishes them
through an API, and serves them to a web client with semantic search powered by
sentence-embedding vectors (pgvector).

```
aggregator  ──fetch──▶  sources.toml (RSS/HTML)  ──▶  SQLite (var/aggregator.db)
    │
    └──publish──▶  translates titles (Google Translate) ──▶  POST /news (server)

server  ──▶  embeds title via Ollama ──▶  Postgres + pgvector ──▶  GET /news (+ search)

client  ──▶  fetches /news, groups by day, text-to-speech playback, semantic search
```

### Components

- **[aggregator](aggregator)** — Rust CLI that fetches news from configured RSS/HTML
  sources into a local SQLite database, translates untranslated titles to English (Google
  Translate API), and publishes them to the server's API.
- **[server](server)** — Axum API server backed by Postgres + [pgvector](https://github.com/pgvector/pgvector).
  Accepts published news (token-authenticated), generates title embeddings via a local
  [Ollama](https://ollama.ai) model, and serves news listing with optional semantic search.
- **[client](client)** — Svelte + Vite frontend that lists news grouped by day, supports
  semantic search, and can read titles aloud via the browser's speech synthesis.

### Getting started

Each component has a devenv config (`server`, `client`) or can be run directly with Cargo/Bun.

**aggregator**

```sh
cd aggregator
# .env holds shared config (DATABASE_URL, NEWS_API_ENDPOINT); create a
# gitignored .env.local with secrets: GOOGLE_TRANSLATE_API_KEY, NEWS_API_TOKEN
make create           # run diesel migrations, creates var/aggregator.db
cargo run -- fetch    # collect news from sources.toml
cargo run -- publish  # translate + publish unpublished items to the server
```

**server**

```sh
cd server
devenv shell            # or install rustc, diesel-cli, postgresql, openssl manually
                         # starts Postgres with pgvector automatically (services.postgres)
# .env holds shared config (DATABASE_URL, POSTGRES_*, SERVER_ADDR, EMBEDDING_MODEL);
# create a gitignored .env.local to override NEWS_API_TOKEN locally
make create             # run diesel migrations
ollama pull <embedding model>   # the model configured as EMBEDDING_MODEL
cargo run
```

**client**

```sh
cd client
devenv shell    # or: bun install
cp .env .env.local   # set VITE_API_URL to the server's address
bun run dev
```

### API

- `GET /news?limit=&search=` — list news, most recent first; `search` runs a semantic
  (vector similarity) query against title embeddings instead of a plain listing.
- `POST /news` — publish a news item (`title`, `pub_date`, `sources`), requires an `auth`
  header matching `NEWS_API_TOKEN`. Used by the aggregator.

