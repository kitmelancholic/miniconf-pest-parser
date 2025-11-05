# miniconf-pest-parser

MiniConf is a plain-text configuration format with sections and `key = value` pairs. This crate provides a PEG grammar written for `pest` and the Rust structures used in the project.

## How it works

- Sections are declared as `[section_name]`.
- Keys use ASCII identifiers and are assigned with `key = value`.
- Comments start with `#` and continue to the end of the line.
- Whitespace (spaces, tabs, newlines) is ignored outside of tokens.

Supported values:

- Strings in double quotes with simple escapes (`\"`, `\n`, `\t`).
- Decimal numbers, optionally signed.
- Booleans (`true`, `false`).
- `null`.
- Arrays like `[value, value]`.
- Objects like `{key: value}`.

All parsed data lands in `Document { sections: HashMap<String, HashMap<String, Value>> }`, so you can read data as `doc.sections["database"]["port"]`.

## Sample

```ini
# Project
title = "Project Alpha"
version = 1.0
flags = { debug: true }

[database]
host = "localhost"
port = 5432
tags = ["primary", "ro"]
```
