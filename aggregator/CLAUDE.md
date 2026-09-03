# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`aggregator` is one component of a larger `news` pipeline (sibling directories `server` and
`client` live one level up, not in this repo checkout). It's a Rust CLI that fetches news from
RSS/HTML sources into a local SQLite database, translates untranslated titles to English via the
Google Translate API, and publishes them to the pipeline's server API.

```
aggregator  ──fetch──▶  sources.toml (RSS/HTML)  ──▶  SQLite (var/aggregator.db)
    │
    └──publish──▶  translates titles (Google Translate) ──▶  POST /news (server)
```

## Commands

```sh
make create           # run diesel migrations, creates var/aggregator.db (needs diesel-cli)
cargo build
cargo run -- fetch     # collect news from all sources in sources.toml
cargo run -- publish   # translate untranslated items and publish them to the server
cargo check
cargo clippy
```

Other Makefile targets: `make clean` (delete all items from the db), `make drop` (delete the db
file), `make reset-republish` (clear `published_id` on all items so `publish` resends them).

There are no automated tests in this crate currently.

### Environment

Config is loaded via `dotenv-flow` (`.env` then `.env.local` overrides), see `src/main.rs`.

- `.env` — shared, checked in: `DATABASE_URL`, `NEWS_API_ENDPOINT`, `RUST_LOG`
- `.env.local` — gitignored, holds secrets: `GOOGLE_TRANSLATE_API_KEY`, `NEWS_API_TOKEN`
- `sources.toml` — gitignored, defines the list of sources `fetch` pulls from (see schema below)

A devenv shell (`devenv.nix`) provides `diesel-cli`, `sqlite`, `openssl`, `pkg-config`,
`rust-analyzer`. `devenv shell` or `nix run` gets you a working toolchain.

## Architecture

Two subcommands, run separately (typically on a schedule): `fetch` collects, `publish` sends.

### `fetch` — `src/collector/`

- `collector::sources::Config` is a tagged enum (`type = "rss" | "html"` in `sources.toml`)
  deserialized from TOML into either `sources::rss::Config` or `sources::html::Config`.
- Both source types implement a `fetch(&Config) -> Result<Data>` that produces a uniform
  `Data { channel: Channel, items: Vec<Item> }`, regardless of source type — this is the
  seam between "how we scrape" and "how we persist."
  - `sources::rss` parses a feed via the `rss` crate; requires `pubDate`/`link`/`title` per item
    and `language` on the channel (`error::IsRequired::required` turns a missing `Option` into an
    `Err`, used throughout to fail fast on malformed feeds).
  - `sources::html` scrapes a page via `scraper`/CSS selectors. Each field to extract is
    declared in `sources.toml` as a selector + extraction strategy (`TextExtractor` gets text or
    an href attribute, `LinkExtractor` resolves relative URLs against the page's base URL,
    `DateExtractor` parses a date with a configurable `chrono` format string). Extraction here
    `.unwrap()`s liberally — a source config with a selector that doesn't match will panic, not
    error gracefully; this is a known-fragile spot when adding/editing HTML sources.
- `collector::collect` (`src/collector/mod.rs`) iterates configured sources, calls `fetch`, then
  `persist`s each `Data`: upserts the `channels` row (on conflicting `link`, just refreshes
  `last_build_date`), then bulk-inserts items with `insert_or_ignore` keyed on `items.guid` — so
  re-running `fetch` is idempotent and only adds genuinely new items. A failure fetching one
  source is swallowed (`let _ = persist(...)`) so one bad source doesn't stop the rest.
- Item `guid` for RSS items is a hash (`sources::rss::hash_id`, `DefaultHasher`) of the feed's
  GUID, or of the link if no GUID is present.

### `publish` — `src/publisher.rs`

Selects items where `published_id IS NULL` (joined with `channels` for `language`), translates
each title to English via `translator::translate` (skipped if `language == "en"`), then
`POST`s to `NEWS_API_ENDPOINT` with an `auth: $NEWS_API_TOKEN` header. On success, the response's
`id` field is stored back into `items.published_id`, marking the item as done — this is what
makes `publish` safe to re-run (only unpublished items are picked up next time).

### Database (`src/schema.rs`, Diesel + SQLite)

Two tables: `channels` (one row per source, unique on `link`) and `items` (one row per news
item, FK to `channel_id`, unique on `guid`, `published_id` nullable — null means "not yet
published"). Schema is Diesel-generated (`diesel.toml` → `src/schema.rs`); after adding a
migration under `migrations/`, run `make create` (or `diesel migration run`) to apply it and
regenerate the schema file — don't hand-edit `schema.rs`.

### Error handling

`anyhow::Result` throughout; no custom error enum beyond `error::IsRequired`, a small extension
trait for turning "this field is required but the source omitted it" `Option`s into errors with
a description of what was missing.
