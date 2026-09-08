# client

Svelte + Vite frontend that fetches published news items from the `server` API, groups them by day (or by search query), and renders them as read/listen cards.

## Project tree

```
client/
├── package.json           # Deps (svelte, vite, pico css, fontawesome) and dev/build/preview scripts
├── package-lock.json       # npm lockfile
├── bun.lockb                # Bun lockfile (bun used to run `dev` in devenv)
├── vite.config.js          # Vite + svelte plugin config; dev server bound to 0.0.0.0
├── svelte.config.js        # Enables vitePreprocess (SCSS/TS in .svelte files)
├── jsconfig.json           # JS type-checking config (checkJs) for editor/LSP support
├── index.html              # Vite entry HTML, mounts #app and loads src/main.js
├── devenv.nix              # devenv shell: language servers + bun-enabled JS toolchain, `run` script = bun dev
├── devenv.lock             # Locked devenv/nix inputs
├── .env                    # VITE_API_URL pointing at the news server API
├── .nvim.lua               # Neovim LSP config (svelte, ts_ls, cssls, html, jsonls)
├── README.md               # Stock create-vite Svelte template README
└── src/
    ├── main.js             # App bootstrap: loads pico/fontawesome CSS + app.scss, mounts App
    ├── App.svelte          # Root component: fetches /news, groups items by day or search, renders list + search box
    ├── Player.svelte       # Play/pause button that reads an item's content aloud via SpeechSynthesis
    ├── Logo.svelte         # Inline SVG brand logo (film-reel style mark)
    ├── app.scss            # Site styling on top of Pico CSS (fonts, card/article layout, round buttons)
    └── vite-env.d.ts        # Ambient type references for svelte/vite client types
```
