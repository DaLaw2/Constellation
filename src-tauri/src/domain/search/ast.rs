//! CQL Abstract Syntax Tree
//!
//! Types representing parsed CQL query expressions.

/// Parsed CQL expression tree.
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    /// Field comparison: field op value
    Comparison {
        field: Field,
        op: ComparisonOp,
        value: Value,
    },
    /// Field IN (value_list)
    InExpr {
        field: Field,
        values: Vec<Value>,
    },
    /// Logical AND
    And(Box<Expr>, Box<Expr>),
    /// Logical OR
    Or(Box<Expr>, Box<Expr>),
    /// Logical NOT (prefix)
    Not(Box<Expr>),
}

/// Known queryable fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Field {
    Tag,
    Name,
    Size,
    Modified,
    Type,
}

impl Field {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "tag" => Some(Field::Tag),
            "name" => Some(Field::Name),
            "size" => Some(Field::Size),
            "modified" => Some(Field::Modified),
            "type" => Some(Field::Type),
            _ => None,
        }
    }
}

/// Comparison operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonOp {
    /// =
    Eq,
    /// !=
    NotEq,
    /// ~ (glob/like)
    Like,
    /// >
    Gt,
    /// <
    Lt,
    /// >=
    Gte,
    /// <=
    Lte,
}

impl ComparisonOp {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "=" => Some(ComparisonOp::Eq),
            "!=" => Some(ComparisonOp::NotEq),
            "~" => Some(ComparisonOp::Like),
            ">" => Some(ComparisonOp::Gt),
            "<" => Some(ComparisonOp::Lt),
            ">=" => Some(ComparisonOp::Gte),
            "<=" => Some(ComparisonOp::Lte),
            _ => None,
        }
    }
}

/// Typed values after parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// String value (from quoted_string)
    String(String),
    /// Numeric value
    Number(f64),
    /// Pre-converted size in bytes (e.g. "10MB" → 10485760)
    SizeBytes(i64),
    /// Pre-converted unix timestamp (e.g. "2024-01-01" → epoch)
    Timestamp(i64),
}
