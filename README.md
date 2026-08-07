# Nitesh Poudel — Portfolio

A static portfolio site built with Rust, [Yew](https://yew.rs/), and [Trunk](https://trunkrs.dev/). The résumé PDF is retained as a static asset for future use.

## Project structure

- `src/app.rs` composes the page from feature components.
- `src/components/` contains isolated presentation and interaction components.
- `src/data/portfolio.rs` owns portfolio content and contact constants.
- `src/services/` wraps browser capabilities, such as clipboard access.
- `src/utils/` contains reusable UI utilities, including theme persistence.
- `styles/` separates theme tokens, global defaults, layout, and component styles.

This keeps content, browser APIs, layout, and UI rendering independent as the site expands.

## Development

Install the WebAssembly target and Trunk once:

```sh
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

Start the development server:

```sh
trunk serve --open
```

Create a production build:

```sh
trunk build --release
```

The production output is written to `dist/`. GitHub Actions builds and deploys that directory to GitHub Pages.
