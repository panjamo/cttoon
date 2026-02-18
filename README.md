# cttoon

A command-line tool that converts JSON to [TOON](https://github.com/toon-format/spec) (Token-Oriented Object Notation) — a compact, human-readable format that reduces token usage by 30–60% compared to JSON.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
# From stdin
echo '{"name":"Alice","age":30}' | cttoon

# From file
cttoon data.json

# With options
cttoon -d pipe --key-folding safe -s 4 data.json
```

## Options

| Option | Short | Description | Default |
|--------|-------|-------------|---------|
| `--delimiter` | `-d` | Delimiter for array values: `comma`, `tab`, `pipe` | `comma` |
| `--spaces` | `-s` | Spaces per indentation level | `2` |
| `--key-folding` | `-k` | Key folding mode: `off`, `safe` | `off` |
| `--flatten-depth` | `-f` | Max depth for key folding | unlimited |

## Examples

### Simple object

```bash
$ echo '{"name":"Alice","age":30}' | cttoon
name: Alice
age: 30
```

### Array of objects (tabular)

```bash
$ echo '{"users":[{"id":1,"name":"Alice"},{"id":2,"name":"Bob"}]}' | cttoon
users[2]{id,name}:
  1,Alice
  2,Bob
```

### Key folding

Collapses single-key object chains into dotted paths:

```bash
$ echo '{"data":{"metadata":{"name":"test"}}}' | cttoon --key-folding safe
data.metadata.name: test
```

Use `--flatten-depth` to limit how many levels get folded:

```bash
$ echo '{"a":{"b":{"c":{"d":"val"}}}}' | cttoon -k safe -f 1
a.b:
  c.d: val
```

### Pipe delimiter

```bash
$ echo '{"items":["x","y","z"]}' | cttoon -d pipe
items[3|]: x|y|z
```

### Root array

```bash
$ echo '[1,2,3]' | cttoon
[3]: 1,2,3
```

## TOON format summary

TOON encodes the JSON data model with minimal syntax:

- **Objects** use indented key-value pairs (`key: value`)
- **Primitive arrays** are inline with length: `tags[3]: a,b,c`
- **Uniform object arrays** use tabular form: `users[2]{id,name}:` followed by rows
- **Strings** are unquoted unless they contain special characters, delimiters, or resemble reserved words/numbers
- **Numbers** use canonical decimal form (no exponents, no trailing zeros)

Full spec: https://github.com/toon-format/spec

## License

MIT
