//! The estate index generator: reads the crate inventory the estate audit
//! maintains (`estate-crates.txt`) plus the live registry data CI fetches
//! (`versions.tsv`), and renders one static page through docs-core — the
//! estate's own markdown engine, consumed here exactly as an outside
//! project would consume it.
//!
//! ```sh
//! cargo run --release -- crates.txt versions.tsv dist/
//! ```

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

/// The peer-audit aggregate and registry bases rendered into the page.
const DOCS_RS: &str = "https://docs.rs";
const REPO_BASE: &str = "https://github.com/WyattAu";

fn main() {
    let mut args = std::env::args().skip(1);
    let crates_txt = PathBuf::from(
        args.next()
            .expect("usage: estate-site <crates.txt> <versions.tsv> <dist-dir>"),
    );
    let versions_tsv = PathBuf::from(
        args.next()
            .expect("usage: estate-site <crates.txt> <versions.tsv> <dist-dir>"),
    );
    let dist = PathBuf::from(
        args.next()
            .expect("usage: estate-site <crates.txt> <versions.tsv> <dist-dir>"),
    );

    let inventory = fs::read_to_string(&crates_txt).expect("read crates.txt");
    let names: Vec<&str> = inventory
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect();

    let mut versions: BTreeMap<String, String> = BTreeMap::new();
    let tsv = fs::read_to_string(&versions_tsv).unwrap_or_default();
    for line in tsv.lines() {
        let mut parts = line.split('\t');
        if let (Some(name), Some(ver)) = (parts.next(), parts.next()) {
            versions.insert(name.to_owned(), ver.to_owned());
        }
    }

    let published: usize = names.iter().filter(|n| versions.contains_key(**n)).count();

    let mut md = String::new();
    md.push_str("# The WyattAu Rust Estate\n\n");
    md.push_str(
        "A layered crate ecosystem: no-std substrates at the bottom,\n\
         domain libraries in the middle, applications on top — every layer\n\
         published to [crates.io](https://crates.io), gated by shared CI\n\
         (fmt, clippy, tests, coverage, miri where it applies, cargo-deny,\n\
         cargo-vet, semver), and composed by an integration suite that runs\n\
         against the published artifacts.\n\n",
    );
    md.push_str(&format!(
        "**{published}** of **{total}** registered crates are live on crates.io.\n\n",
        total = names.len()
    ));
    md.push_str("| crate | version | docs | source |\n|---|---|---|---|\n");
    for name in &names {
        match versions.get(*name) {
            Some(ver) => {
                md.push_str(&format!(
                    "| [`{name}`]({DOCS_RS}/{name}) | `{ver}` | [docs]({DOCS_RS}/{name}) | [source]({REPO_BASE}/{name}) |\n"
                ));
            }
            None => {
                md.push_str(&format!(
                    "| `{name}` | *pre-release* | — | [source]({REPO_BASE}/{name}) |\n"
                ));
            }
        }
    }
    md.push_str("\n## Layers\n\n");
    md.push_str("- **L0** — no-std leaves that take untrusted bytes (`can-core`, `dbc-parse`, `mp4-core`, `id3-core`, …)\n");
    md.push_str(
        "- **L1** — domain models over the leaves (`audiobook-core`, `cal-model`, `wire-kit`, …)\n",
    );
    md.push_str("- **L2** — application stacks and services (`outbox-kit`, `ledger-kit`, `audiobook-shelf`, …)\n");
    md.push_str("- **L3** — the tools that keep the estate honest (`estate-integration`, `font-conformance`, `engineering-standards`)\n");

    let html =
        docs_core::render(&md, "/").expect("the estate's own engine renders the estate's own page");
    fs::create_dir_all(&dist).expect("create dist");
    let page = format!(
        "<!DOCTYPE html>\n<html lang=\"en\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\
         <title>The WyattAu Rust Estate</title>\
         <style>body{{font-family:system-ui,sans-serif;max-width:60rem;margin:2rem auto;padding:0 1rem;line-height:1.5}}\
         table{{border-collapse:collapse;width:100%}}td,th{{border:1px solid #ddd;padding:.35rem .6rem;text-align:left}}\
         tr:nth-child(even){{background:#fafafa}}a{{color:#0366d6}}</style>\
         </head><body>\n{html}\n</body></html>\n"
    );
    fs::write(dist.join("index.html"), page).expect("write index.html");
    println!(
        "wrote {} ({published} published crates)",
        dist.join("index.html").display()
    );
}
