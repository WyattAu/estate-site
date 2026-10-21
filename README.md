# estate-site

The WyattAu Rust estate's index page, rendered by
[docs-core](https://docs.rs/docs-core) — the estate's own markdown
engine, consumed here exactly as an outside project would consume the
published crate.

The generator reads the crate inventory the estate audit maintains
(`estate-crates.txt` in engineering-standards) plus live registry
versions fetched at build time, renders one static page, and deploys it
to GitHub Pages weekly and on every push.

## Layout

- `src/main.rs` — the generator: inventory + versions → markdown → docs-core → HTML
- `.github/workflows/site.yml` — fetch, build, deploy (Mondays + push + dispatch)
- `.github/workflows/ci.yml` — the estate gates (rust-kit, tier c)
