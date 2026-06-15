# Rust + Leptos Portfolio

A full-stack portfolio website built entirely in **Rust** using the [Leptos](https://leptos.dev) web framework with [Axum](https://github.com/tokio-rs/axum) for server-side rendering. Styled with [TailwindCSS](https://tailwindcss.com).

## Architecture

This project is deployed to **Cloudflare Pages via Cloudflare Workers**.
- **SSR**: The Axum server is compiled to WebAssembly and runs in a Cloudflare Worker intercepting `fetch` events to server-side render the initial HTML.
- **Hydration**: A separate client WebAssembly bundle takes over in the browser for interactivity.
- **Routing**: Handled natively by `leptos_router`.
- **Styling**: TailwindCSS scans `.rs` files for utility classes and compiles to `.css`.

## Prerequisites

```bash
# Install Rust nightly + WASM target
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown

# Install the WASM binding generator CLI
cargo install wasm-bindgen-cli --version 0.2.125 --locked

# Install Cloudflare Wrangler and TailwindCSS
npm ci
```

## Development

Since this project runs natively in WebAssembly via Cloudflare Workers, we use a custom bash script instead of `cargo leptos`.

```bash
# 1. Compile both the SSR Worker and Client WASM into the site/ directory
npm run build

# 2. Start the local Cloudflare Wrangler dev server (http://127.0.0.1:8788)
npm run dev
```

## Production Build

```bash
# Authenticate once, recreate the deleted Pages project, then deploy
npm run cf:login
npm run cf:project:create
npm run deploy
```

The Pages project is configured in `wrangler.jsonc` as `shriyans-arkal`,
with `main` used as the production branch. Wrangler's browser login stores
Cloudflare credentials outside this repository; no API token belongs in source
control.

Pushes to `main` run `.github/workflows/deploy.yml`. The repository must have
`CLOUDFLARE_ACCOUNT_ID` and `CLOUDFLARE_API_TOKEN` Actions secrets; the token
needs the `Account > Cloudflare Pages > Edit` permission.

## Project Structure

```
src/
├── bin/
│   ├── server.rs        # Cloudflare Worker SSR entry point
│   └── client.rs        # Browser Hydration entry point
├── app.rs               # Root component + routing
├── pages/
│   ├── home.rs          # Landing page
│   ├── projects.rs      # Project showcase
│   ├── background.rs    # Experience & Education timeline
│   ├── resume.rs        # Resume viewer
│   └── contact.rs       # Contact form
└── components/
    ├── sidebar_left.rs  # Main navigation
    └── theme_switcher.rs # Global light/dark/solarized theme toggle
style/
└── tailwind.css          # TailwindCSS entry
site/                     # Generated build output
build.sh                  # Custom compilation script
```
