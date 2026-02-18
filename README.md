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

## PowerShell: ConvertTo-Toon

A `ConvertTo-Toon` cmdlet is included in `ConvertTo-Toon.ps1`. It pipes any PowerShell object through `ConvertTo-Json` and then `cttoon`, so you can use it directly in the pipeline.

### Setup

Copy the script into your PowerShell modules directory and dot-source it from your profile:

```powershell
# Copy the script
Copy-Item ConvertTo-Toon.ps1 "$HOME\Documents\PowerShell\Scripts\ConvertTo-Toon.ps1"

# Add to your profile (run once)
Add-Content $PROFILE '. "$HOME\Documents\PowerShell\Scripts\ConvertTo-Toon.ps1"'
```

After restarting PowerShell, `ConvertTo-Toon` is available in every session.

### Parameters

| Parameter | Description | Default |
|-----------|-------------|---------|
| `-InputObject` | Input from pipeline | — |
| `-Depth` | JSON serialization depth | `2` |
| `-Delimiter` | `comma`, `tab`, or `pipe` | `comma` |
| `-Spaces` | Indentation spaces per level | `2` |
| `-KeyFolding` | `off`, `on`, or `safe` (`on` maps to `safe`) | `off` |
| `-FlattenDepth` | Max key folding depth | unlimited |
| `-OutFile` | Write output to file instead of stdout | — |

### Examples

```powershell
# Convert a hashtable
@{ name = "Alice"; age = 30 } | ConvertTo-Toon

# Convert command output with deeper nesting
Get-Process | Select-Object -First 3 Name, Id, CPU | ConvertTo-Toon -Depth 4

# Use pipe delimiter and write to file
Get-Service | Select-Object Name, Status | ConvertTo-Toon -Delimiter pipe -OutFile services.toon

# With key folding
@{ data = @{ metadata = @{ name = "test" } } } | ConvertTo-Toon -KeyFolding safe
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
