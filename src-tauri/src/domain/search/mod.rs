//! CQL Search Module
//!
//! Pest-based parser for CQL (Constellation Query Language) queries.
//! Parses query strings into an AST for SQL generation.

pub mod ast;
pub mod error;
pub mod parser;

pub use ast::{ComparisonOp, Expr, Field, Value};
pub use error::CqlParseError;
pub use parser::parse_cql;
