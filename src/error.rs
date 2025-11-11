use std::fmt;

use thiserror::Error;

use crate::parser::Rule;

/// Errors produced while parsing MiniConf documents.
#[derive(Debug, Error)]
pub enum MiniConfError {
    /// Underlying pest error emitted when the grammar fails.
    #[error("parse error: {0}")]
    Pest(Box<pest::error::Error<Rule>>),
    /// Higher-level semantic issue surfaced after the grammar matched.
    #[error("{kind} on line {line}: {message}")]
    Semantic {
        /// Classification of the problem.
        kind: ParseErrorKind,
        /// 1-based line number where the issue originated.
        line: usize,
        /// Human-friendly description.
        message: String,
    },
}

impl MiniConfError {
    /// Creates a semantic error for convenience.
    pub(crate) fn semantic(kind: ParseErrorKind, line: usize, message: impl Into<String>) -> Self {
        Self::Semantic {
            kind,
            line,
            message: message.into(),
        }
    }
}

impl From<pest::error::Error<Rule>> for MiniConfError {
    fn from(value: pest::error::Error<Rule>) -> Self {
        Self::Pest(Box::new(value))
    }
}

/// Enum describing semantic parsing problems.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ParseErrorKind {
    /// Duplicate key encountered inside a section.
    DuplicateKey,
    /// Value failed validation after the grammar matched.
    InvalidValue,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateKey => write!(f, "duplicate key"),
            Self::InvalidValue => write!(f, "invalid value"),
        }
    }
}
