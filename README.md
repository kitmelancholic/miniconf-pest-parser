# miniconf-parser

MiniConf is a plain-text configuration format with simple sections and `key = value` pairs. This crate provides both a reusable parser library and a convenient CLI built on top of a `pest` grammar.

## Features
- Declarative PEG grammar in `src/grammar.pest` with explicit support for sections, key/value pairs, comments, and arrays.
- Strongly-typed AST (`Document`, `Section`, `Value`) re-exported by the library.
- Helpful diagnostics powered by `thiserror` with duplicate-key detection.
- Batteries-included CLI for parsing or validating files plus JSON/pretty output modes.

## Quick Start
### Library
```rust
use miniconf_parser::parse_str;

let doc = parse_str("[service]\nport = 8080\n")?;
println!("sections: {}", doc.sections().count());
```

### CLI
```shell
miniconf-parser parse config.mc            # pretty print
miniconf-parser parse config.mc -f json    # emit JSON
miniconf-parser check config.mc            # validate only
```

## Grammar Highlights
1. `section_header`: `[section_name]` with alphanumeric, `_`, or `-` characters.
2. `key_value`: `key = value` with flexible whitespace and optional trailing comments.
3. `comment_line`: `# comment` to end-of-line, including inline comments.
4. `value`: quoted/bare strings, integers/floats, booleans (`true/false/yes/no`), and arrays like `[one, two]`.

## Development Workflow
- `make fmt` – format the codebase.
- `make clippy` – run `cargo clippy -- -D warnings` to keep lints clean.
- `make test` – execute the full test suite in `tests/` plus doctests.
- `make doc` – build API docs (`cargo doc --no-deps`).
- `make publish` – sanity-check `cargo publish --dry-run` before releasing.

The test suite combines parser-focused cases (`tests/parser_tests.rs`) with CLI-level integration tests. Shared fixtures live under `tests/common/`. Examples in `examples/` demonstrate typical embedding scenarios.

## Documentation
Comprehensive rustdoc comments and doctests are enabled on docs.rs. Run `cargo doc --open` for a local copy. The CLI is implemented with `clap` 4.5, while the library exposes ergonomic error types so downstream applications can surface helpful diagnostics.
