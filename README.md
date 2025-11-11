# colors

A small Rust CLI that converts hex color codes to RGB.

- Accepts optional leading `#`.
- Supports 3-digit (`#RGB`) and 6-digit (`#RRGGBB`) hex forms.
- Outputs `rgb(R, G, B)` per valid input.
- Prints errors to stderr and exits with code 1 if any input is invalid.

## Build

```
cargo build
```

Or run directly with Cargo:

```
cargo run -- <HEX>...
```

PowerShell note: quote any argument starting with `#`, e.g. `"#abc"`.

## Usage

```
colors <HEX>...
```

- `<HEX>`: one or more hex color codes (3 or 6 hex digits), with or without a leading `#`.

### Examples

Multiple valid inputs:

```
cargo run -- 112233 ff00aa "#abc"
```

Output:
```
rgb(17, 34, 51)
rgb(255, 0, 170)
rgb(170, 187, 204)
```

Quoted `#` and uppercase shorthand:

```
cargo run -- "#112233" 0F0
```

Output:
```
rgb(17, 34, 51)
rgb(0, 255, 0)
```

Mixed valid/invalid (non-zero exit code):

```
cargo run -- ggg 1234 00ff00
```

Stderr:
```
ggg: invalid hex digit 'g' at position 1
1234: expected 3 or 6 hex digits, got 4
```

Stdout:
```
rgb(0, 255, 0)
```

## Help

```
cargo run -- --help
```

## Tests

```
cargo test
```
