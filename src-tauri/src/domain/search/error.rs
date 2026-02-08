//! CQL Parse Errors
//!
//! Error types for CQL query parsing and validation.

use std::fmt;

#[derive(Debug, Clone)]
pub enum CqlParseError {
    /// Query string is empty
    EmptyQuery,
    /// Syntax error from pest parser
    SyntaxError(String),
    /// Unknown field name
    InvalidField(String),
    /// Invalid size literal (e.g. "10XB")
    InvalidSize(String),
    /// Invalid date format (expected YYYY-MM-DD)
    InvalidDate(String),
    /// Operator not supported for the given field
    InvalidOperator { field: String, op: String },
    /// Internal parser error (grammar/AST mismatch - should never occur)
    InternalError(String),
}

impl fmt::Display for CqlParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CqlParseError::EmptyQuery => write!(f, "Query is empty"),
            CqlParseError::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
            CqlParseError::InvalidField(name) => write!(f, "Unknown field: {}", name),
            CqlParseError::InvalidSize(val) => write!(f, "Invalid size value: {}", val),
            CqlParseError::InvalidDate(val) => {
                write!(f, "Invalid date (expected YYYY-MM-DD): {}", val)
            }
            CqlParseError::InvalidOperator { field, op } => {
                write!(
                    f,
                    "Operator '{}' is not supported for field '{}'",
                    op, field
                )
            }
            CqlParseError::InternalError(msg) => {
                write!(f, "Internal parser error: {}", msg)
            }
        }
    }
}

impl std::error::Error for CqlParseError {}
