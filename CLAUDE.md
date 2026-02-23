# cttoon

Command-line tool to convert JSON or XML to TOON (Token-Oriented Object Notation). Input format is auto-detected: input starting with `<` is treated as XML and converted to JSON internally before encoding.

## Build & Run

```bash
cargo build
cargo run -- --help
echo '{"name":"Alice","age":30}' | cargo run
echo '<person><name>Alice</name></person>' | cargo run
cargo run -- input.json
cargo run -- input.xml
```

## Test

```bash
cargo test
```

## Lint & Format

```bash
cargo clippy --fix
cargo fmt
```

## Project Structure

- `src/main.rs` — CLI entry point, XML auto-detection, XML→JSON converter, delegates encoding to `format-as-toon` crate
- `Cargo.toml` — Dependencies: `clap` (CLI), `serde_json` (JSON parsing), `quick-xml` (XML parsing), `format-as-toon` (TOON encoding)

## CLI Options

| Flag | Description | Default |
|------|-------------|---------|
| `[INPUT]` | JSON or XML file path (stdin if omitted; format auto-detected) | stdin |
| `-d, --delimiter` | `comma`, `tab`, or `pipe` | `comma` |
| `-s, --spaces` | Indentation spaces per level | `2` |
| `-k, --key-folding` | `off` or `safe` (dotted-path collapsing) | `off` |
| `-f, --flatten-depth` | Max key folding depth | unlimited |

## TOON Format

TOON (Token-Oriented Object Notation) is a compact, human-readable encoding of the JSON data model.
Spec: https://github.com/toon-format/spec
