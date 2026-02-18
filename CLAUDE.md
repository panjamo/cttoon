# cttoon

Command-line tool to convert JSON to TOON (Token-Oriented Object Notation).

## Build & Run

```bash
cargo build
cargo run -- --help
echo '{"name":"Alice","age":30}' | cargo run
cargo run -- input.json
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

- `src/main.rs` — CLI entry point + TOON encoder (hand-rolled per spec, no external TOON crate)
- `Cargo.toml` — Dependencies: `clap` (CLI), `serde_json` (JSON parsing)

## CLI Options

| Flag | Description | Default |
|------|-------------|---------|
| `[INPUT]` | JSON file path (stdin if omitted) | stdin |
| `-d, --delimiter` | `comma`, `tab`, or `pipe` | `comma` |
| `-s, --spaces` | Indentation spaces per level | `2` |
| `-k, --key-folding` | `off` or `safe` (dotted-path collapsing) | `off` |
| `-f, --flatten-depth` | Max key folding depth | unlimited |

## TOON Format

TOON (Token-Oriented Object Notation) is a compact, human-readable encoding of the JSON data model.
Spec: https://github.com/toon-format/spec
