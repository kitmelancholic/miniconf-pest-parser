#![doc = include_str!("../docs.md")]
#![deny(unsafe_code)]
#![warn(missing_docs)]

/// Abstract syntax tree types used to represent parsed documents.
pub mod ast;
/// Error types emitted by the parser.
pub mod error;
/// Low-level parser utilities and the generated pest machinery.
#[allow(missing_docs)]
pub mod parser;

pub use ast::{Document, Entry, Section, Value};
pub use error::{MiniConfError, ParseErrorKind};

/// Convenience function that parses `source` into a [`Document`].
pub fn parse_str(source: &str) -> Result<Document, MiniConfError> {
    parser::parse_document(source)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_document() {
        let doc = parse_str("key = value\n").expect("parsed");
        let root = doc.section("root").expect("root section");
        assert_eq!(root.entries[0].key, "key");
    }
}
