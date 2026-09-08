# aggregator

Rust CLI that fetches news from RSS/HTML sources, extracts article content, translates non-English items via a local Ollama model, and publishes them to a remote news API.

## Project tree

```
aggregator/
├── Cargo.toml            # Crate manifest (diesel/sqlite, reqwest, rss, scraper, readability, etc.)
├── diesel.toml           # Diesel CLI config (schema output path, migrations dir)
├── devenv.nix            # devenv shell: rust toolchain, sqlite, diesel-cli, ollama process
├── devenv.lock           # Locked devenv/nix inputs
├── Makefile              # DB/task shortcuts (create/clean/drop/show, create-translator)
├── Modelfile             # Ollama model definition for the translator (gemma-based, EN-only output)
├── .env                  # Local environment vars (DATABASE_URL, API endpoint/token, ...)
├── .nvim.lua             # Neovim project-local config
├── migrations/           # Diesel SQL migrations for the sqlite schema
│   ├── 2023-12-11-100421_create_news/     # Initial channels/items tables
│   └── 2026-09-03-191157-0000_add_content_to_items/  # Adds items.content column
└── src/
    ├── main.rs           # CLI entrypoint (clap): `fetch` and `publish` subcommands, loads sources.toml
    ├── schema.rs          # Diesel-generated schema (channels, items tables) — do not hand-edit
    ├── error.rs           # `IsRequired` helper trait: Option -> Result with a descriptive error
    ├── translator.rs      # Translates text to English by calling a local Ollama /api/generate endpoint
    ├── publisher.rs       # Publishes unpublished items: translates if needed, POSTs to NEWS_API_ENDPOINT
    └── collector/
        ├── mod.rs         # Orchestrates fetch-per-source + upsert channels/items into sqlite
        ├── content.rs     # Downloads an article page and extracts readable body text
        └── sources/
            ├── mod.rs     # Shared Config/Channel/Item types; dispatches to rss or html fetcher
            ├── rss.rs     # RSS feed source: parses feed, hashes guids, builds Channel/Item data
            └── html.rs    # Generic HTML-scraping source driven by CSS-selector config
```
