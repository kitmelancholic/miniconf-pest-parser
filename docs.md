# MiniConf Parser Documentation

MiniConf is a lightweight, human-friendly configuration format that keeps everything in plain text. This crate bundles a reusable parser library, a reference command-line interface, and the original `pest` grammar so you can validate or embed MiniConf in your own tools with zero boilerplate.

## Feature Highlights
- ergonomic AST (`Document`, `Section`, `Value`) with iterator helpers for traversing sections and entries
- duplicate-key detection plus clear diagnostics powered by `thiserror`
- flexible value parsing: bare/quoted strings, integers, floats, booleans (`true/false/yes/no`), and arrays like `[one, two]`
- batteries-included CLI for parsing, validating, and exporting as pretty text or JSON

## Installation
Library users can add the dependency with:

```sh
cargo add miniconf-parser
```

To install the CLI:

```sh
cargo install miniconf-parser
```

`cargo install --path .` works when hacking locally.

## Library Quick Start
```rust
use miniconf_parser::parse_str;

let source = "[service]\nport = 8080\n";
let doc = parse_str(source)?;
let service = doc.section("service").unwrap();
assert_eq!("8080", service.get("port").unwrap().as_str());
```

Every parse error provides byte offsets and friendly messages, making it easy to surface diagnostics in editors or servers.

## CLI Usage
- `miniconf-parser parse config.mc` – parse the file and pretty-print sections and key/value pairs.
- `miniconf-parser parse config.mc -f json -o config.json` – export the AST as JSON to a file.
- `miniconf-parser check config.mc` – validate without printing; exits non-zero on failure.

Passing `--help` reveals all options, and both commands accept relative or absolute paths. The CLI prints colored checkmarks when validation succeeds.

## Grammar Overview
The PEG grammar lives in `src/grammar.pest`. Key rules:
1. `section_header = { "[" ~ ASCII_ALPHANUMERIC+ ~ ( "-" | "_" )* ~ "]" }`
2. `key_value = { key ~ "=" ~ value ~ comment? }`
3. `comment = { "#" ~ (!NEWLINE ~ ANY)* }`
Feel free to fork the grammar to extend the format; recompiling via `cargo build` regenerates the parser.

## Development Hints
For contributors, the `Makefile` exposes `make fmt`, `make clippy`, `make test`, and `make doc`. Integration tests live under `tests/`, while runnable examples sit in `examples/` for quick experimentation.
