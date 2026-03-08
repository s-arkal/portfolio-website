# Rust + Leptos Portfolio

A full-stack portfolio website built entirely in **Rust** using the [Leptos](https://leptos.dev) web framework with [Axum](https://github.com/tokio-rs/axum) for server-side rendering. Styled with [TailwindCSS](https://tailwindcss.com).

## Architecture

One Rust binary handles everything:
- **SSR**: Server-side renders the initial HTML for fast page loads and SEO
- **Hydration**: WASM takes over in the browser for interactivity
- **Routing**: Client-side navigation via `leptos_router`
- **Styling**: TailwindCSS scans `.rs` files for utility classes

## Prerequisites

```bash
# Install Rust nightly + WASM target
rustup toolchain install nightly --allow-downgrade
rustup target add wasm32-unknown-unknown

# Install cargo-leptos
cargo install cargo-leptos --locked

# Install npm dependencies (TailwindCSS)
npm install
```

## Development

```bash
# Start dev server with hot-reload at http://127.0.0.1:3000
cargo leptos watch
```

## Production Build

```bash
cargo leptos build --release
```

The compiled binary and static assets will be in `target/site/`.

## Project Structure

```
src/
├── main.rs              # Axum server entry point
├── lib.rs               # WASM hydration entry point
├── app.rs               # Root component + routing
├── pages/
│   ├── home.rs          # Landing page
│   ├── projects.rs      # Project showcase
│   ├── blog.rs          # Blog listing
│   └── contact.rs       # Contact form
└── components/
    ├── navbar.rs         # Navigation bar
    └── footer.rs         # Footer
style/
└── tailwind.css          # TailwindCSS entry
public/                   # Static assets
```
