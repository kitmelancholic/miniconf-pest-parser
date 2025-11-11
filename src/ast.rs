use std::collections::BTreeMap;

use serde::Serialize;

/// Parsed MiniConf document consisting of named sections.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Document {
    sections: BTreeMap<String, Section>,
}

impl Document {
    /// Creates an empty [`Document`].
    pub fn new() -> Self {
        Self {
            sections: BTreeMap::new(),
        }
    }

    /// Returns a section by name, creating it when absent.
    pub(crate) fn ensure_section_mut(&mut self, name: &str) -> &mut Section {
        self.sections
            .entry(name.to_string())
            .or_insert_with(|| Section::new(name))
    }

    /// Returns an immutable view of a section, if present.
    pub fn section(&self, name: &str) -> Option<&Section> {
        self.sections.get(name)
    }

    /// Returns an iterator over all sections, ordered by name.
    pub fn sections(&self) -> impl Iterator<Item = (&String, &Section)> + '_ {
        self.sections.iter()
    }
}

impl Default for Document {
    fn default() -> Self {
        Self::new()
    }
}

/// A section groups related key/value entries.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Section {
    /// Section identifier as it appeared in the file.
    pub name: String,
    /// Key/value entries in declaration order.
    pub entries: Vec<Entry>,
}

impl Section {
    /// Creates a section with no entries.
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            entries: Vec::new(),
        }
    }
}

/// Single key/value assignment inside a section.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Entry {
    /// Logical key.
    pub key: String,
    /// Parsed value.
    pub value: Value,
    /// 1-based line number, useful for diagnostics.
    pub line: usize,
}

impl Entry {
    /// Convenience constructor for tests and parser.
    pub fn new(key: impl Into<String>, value: Value, line: usize) -> Self {
        Self {
            key: key.into(),
            value,
            line,
        }
    }
}

/// Supported MiniConf values.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Value {
    /// Quoted or bare string data.
    String(String),
    /// Integer or floating-point number.
    Number(f64),
    /// Boolean (`true`, `false`, `yes`, `no`).
    Bool(bool),
    /// Ordered list of nested values.
    Array(Vec<Value>),
}

impl Value {
    /// Returns the string representation when the value is textual.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(s) => Some(s),
            _ => None,
        }
    }
}
