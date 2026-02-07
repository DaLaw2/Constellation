//! CQL Parser
//!
//! Parses CQL query strings into AST using pest, with semantic validation
//! and value conversion (size literals, date strings).

use pest::Parser;
use pest_derive::Parser;

use super::ast::{ComparisonOp, Expr, Field, Value};
use super::error::CqlParseError;

#[derive(Parser)]
#[grammar = "domain/search/query.pest"]
struct CqlParser;

/// Parses a CQL query string into an AST expression.
pub fn parse_cql(input: &str) -> Result<Expr, CqlParseError> {
    let input = input.trim();
    if input.is_empty() {
        return Err(CqlParseError::EmptyQuery);
    }

    let pairs = CqlParser::parse(Rule::query, input)
        .map_err(|e| CqlParseError::SyntaxError(format_pest_error(e)))?;

    let query_pair = pairs.into_iter().next().unwrap();
    let expr_pair = query_pair
        .into_inner()
        .find(|p| p.as_rule() == Rule::expression)
        .unwrap();

    let expr = build_expression(expr_pair)?;
    validate_semantics(&expr)?;
    Ok(expr)
}

/// Formats a pest error into a user-friendly string.
fn format_pest_error(e: pest::error::Error<Rule>) -> String {
    let msg = match &e.variant {
        pest::error::ErrorVariant::ParsingError {
            positives,
            negatives,
        } => {
            let expected: Vec<String> = positives.iter().map(|r| format!("{:?}", r)).collect();
            let unexpected: Vec<String> = negatives.iter().map(|r| format!("{:?}", r)).collect();
            let mut parts = Vec::new();
            if !expected.is_empty() {
                parts.push(format!("expected {}", expected.join(", ")));
            }
            if !unexpected.is_empty() {
                parts.push(format!("unexpected {}", unexpected.join(", ")));
            }
            parts.join("; ")
        }
        pest::error::ErrorVariant::CustomError { message } => message.clone(),
    };

    let location = match e.location {
        pest::error::InputLocation::Pos(pos) => format!(" at position {}", pos),
        pest::error::InputLocation::Span((start, end)) => {
            format!(" at position {}-{}", start, end)
        }
    };

    format!("{}{}", msg, location)
}

/// Builds an expression AST from a pest expression pair (handles OR).
fn build_expression(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    let mut left = build_and_expr(first)?;

    while let Some(next) = inner.next() {
        let right = build_and_expr(next)?;
        left = Expr::Or(Box::new(left), Box::new(right));
    }

    Ok(left)
}

/// Builds an AND expression from a pest and_expr pair.
fn build_and_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();
    let mut left = build_unary_expr(first)?;

    while let Some(next) = inner.next() {
        let right = build_unary_expr(next)?;
        left = Expr::And(Box::new(left), Box::new(right));
    }

    Ok(left)
}

/// Builds a unary (NOT or primary) expression.
fn build_unary_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    let mut inner = pair.into_inner();
    let first = inner.next().unwrap();

    match first.as_rule() {
        Rule::not_op => {
            let operand = inner.next().unwrap();
            let expr = build_unary_expr(operand)?;
            Ok(Expr::Not(Box::new(expr)))
        }
        _ => build_primary(first),
    }
}

/// Builds a primary expression (comparison, in_expr, or grouped expression).
fn build_primary(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    match pair.as_rule() {
        Rule::primary => {
            let inner = pair.into_inner().next().unwrap();
            build_primary(inner)
        }
        Rule::expression => build_expression(pair),
        Rule::comparison => build_comparison(pair),
        Rule::in_expr => build_in_expr(pair),
        _ => Err(CqlParseError::SyntaxError(format!(
            "Unexpected rule: {:?}",
            pair.as_rule()
        ))),
    }
}

/// Builds a comparison expression (field op value).
fn build_comparison(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    let mut inner = pair.into_inner();

    let field_pair = inner.next().unwrap();
    let field = parse_field(field_pair.as_str())?;

    let op_pair = inner.next().unwrap();
    let op_str = op_pair.as_str();
    let op = ComparisonOp::from_str(op_str)
        .ok_or_else(|| CqlParseError::SyntaxError(format!("Unknown operator: {}", op_str)))?;

    let value_pair = inner.next().unwrap();
    let value = parse_value(value_pair, field)?;

    Ok(Expr::Comparison { field, op, value })
}

/// Builds an IN expression (field IN (values...)).
fn build_in_expr(pair: pest::iterators::Pair<Rule>) -> Result<Expr, CqlParseError> {
    let mut inner = pair.into_inner();

    let field_pair = inner.next().unwrap();
    let field = parse_field(field_pair.as_str())?;

    // Skip in_op if present as a named rule (it might be silent)
    // Next should be value_list
    let value_list_pair = inner.next().unwrap();

    let values: Result<Vec<Value>, CqlParseError> = value_list_pair
        .into_inner()
        .map(|v| parse_value(v, field))
        .collect();

    Ok(Expr::InExpr {
        field,
        values: values?,
    })
}

/// Parses a field name string into a Field enum.
fn parse_field(s: &str) -> Result<Field, CqlParseError> {
    Field::from_str(s).ok_or_else(|| CqlParseError::InvalidField(s.to_string()))
}

/// Parses a value pair, using field context for type coercion.
fn parse_value(pair: pest::iterators::Pair<Rule>, field: Field) -> Result<Value, CqlParseError> {
    let inner = pair.into_inner().next().unwrap();

    match inner.as_rule() {
        Rule::quoted_string => {
            let raw = inner.into_inner().next().unwrap().as_str();
            let unescaped = unescape_string(raw);

            // For modified field, try to parse as date
            if field == Field::Modified {
                let ts = parse_date_to_timestamp(&unescaped)?;
                Ok(Value::Timestamp(ts))
            } else {
                Ok(Value::String(unescaped))
            }
        }
        Rule::size_literal => {
            let bytes = parse_size_to_bytes(inner.as_str())?;
            Ok(Value::SizeBytes(bytes))
        }
        Rule::number => {
            let n: f64 = inner
                .as_str()
                .parse()
                .map_err(|_| CqlParseError::SyntaxError("Invalid number".to_string()))?;

            // For size field, treat raw number as bytes
            if field == Field::Size {
                Ok(Value::SizeBytes(n as i64))
            } else if field == Field::Modified {
                Ok(Value::Timestamp(n as i64))
            } else {
                Ok(Value::Number(n))
            }
        }
        _ => Err(CqlParseError::SyntaxError(format!(
            "Unexpected value rule: {:?}",
            inner.as_rule()
        ))),
    }
}

/// Unescapes a string (handles \\, \", \n, \t).
fn unescape_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            match chars.next() {
                Some('"') => result.push('"'),
                Some('\\') => result.push('\\'),
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some(other) => {
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(ch);
        }
    }
    result
}

/// Parses a size literal (e.g. "10MB") to bytes.
fn parse_size_to_bytes(s: &str) -> Result<i64, CqlParseError> {
    let upper = s.to_uppercase();

    let (num_str, multiplier) = if upper.ends_with("GB") {
        (&s[..s.len() - 2], 1_073_741_824i64)
    } else if upper.ends_with("MB") {
        (&s[..s.len() - 2], 1_048_576i64)
    } else if upper.ends_with("KB") {
        (&s[..s.len() - 2], 1_024i64)
    } else if upper.ends_with('B') {
        (&s[..s.len() - 1], 1i64)
    } else {
        return Err(CqlParseError::InvalidSize(s.to_string()));
    };

    let num: f64 = num_str
        .parse()
        .map_err(|_| CqlParseError::InvalidSize(s.to_string()))?;

    Ok((num * multiplier as f64) as i64)
}

/// Parses a date string "YYYY-MM-DD" to unix timestamp (UTC midnight).
fn parse_date_to_timestamp(s: &str) -> Result<i64, CqlParseError> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(CqlParseError::InvalidDate(s.to_string()));
    }

    let year: i32 = parts[0]
        .parse()
        .map_err(|_| CqlParseError::InvalidDate(s.to_string()))?;
    let month: u32 = parts[1]
        .parse()
        .map_err(|_| CqlParseError::InvalidDate(s.to_string()))?;
    let day: u32 = parts[2]
        .parse()
        .map_err(|_| CqlParseError::InvalidDate(s.to_string()))?;

    if month < 1 || month > 12 || day < 1 || day > 31 || year < 1970 {
        return Err(CqlParseError::InvalidDate(s.to_string()));
    }

    Ok(ymd_to_unix(year, month, day))
}

/// Converts a date (YYYY, MM, DD) to unix timestamp (UTC midnight).
/// Uses Howard Hinnant's civil_from_days algorithm.
fn ymd_to_unix(year: i32, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year } as i64;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = (y - era * 400) as u32;
    let m = month;
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146097 + doe as i64 - 719468;
    days * 86400
}

/// Validates semantic correctness of the AST (field/operator compatibility).
fn validate_semantics(expr: &Expr) -> Result<(), CqlParseError> {
    match expr {
        Expr::Comparison { field, op, .. } => validate_field_op(*field, *op),
        Expr::InExpr { field, .. } => {
            // IN is only valid for tag and name
            match field {
                Field::Tag | Field::Name | Field::Type => Ok(()),
                _ => Err(CqlParseError::InvalidOperator {
                    field: format!("{:?}", field).to_lowercase(),
                    op: "IN".to_string(),
                }),
            }
        }
        Expr::And(left, right) | Expr::Or(left, right) => {
            validate_semantics(left)?;
            validate_semantics(right)
        }
        Expr::Not(inner) => validate_semantics(inner),
    }
}

/// Validates that an operator is supported for a given field.
fn validate_field_op(field: Field, op: ComparisonOp) -> Result<(), CqlParseError> {
    let valid = match field {
        Field::Tag => matches!(
            op,
            ComparisonOp::Eq | ComparisonOp::NotEq | ComparisonOp::Like
        ),
        Field::Name => matches!(
            op,
            ComparisonOp::Eq | ComparisonOp::NotEq | ComparisonOp::Like
        ),
        Field::Size => matches!(
            op,
            ComparisonOp::Eq
                | ComparisonOp::NotEq
                | ComparisonOp::Gt
                | ComparisonOp::Lt
                | ComparisonOp::Gte
                | ComparisonOp::Lte
        ),
        Field::Modified => matches!(
            op,
            ComparisonOp::Eq
                | ComparisonOp::NotEq
                | ComparisonOp::Gt
                | ComparisonOp::Lt
                | ComparisonOp::Gte
                | ComparisonOp::Lte
        ),
        Field::Type => matches!(op, ComparisonOp::Eq | ComparisonOp::NotEq),
    };

    if valid {
        Ok(())
    } else {
        Err(CqlParseError::InvalidOperator {
            field: format!("{:?}", field).to_lowercase(),
            op: format!("{:?}", op),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_tag_eq() {
        let expr = parse_cql(r#"tag = "vacation""#).unwrap();
        assert!(matches!(
            expr,
            Expr::Comparison {
                field: Field::Tag,
                op: ComparisonOp::Eq,
                ..
            }
        ));
    }

    #[test]
    fn parse_and_expression() {
        let expr = parse_cql(r#"tag = "vacation" AND tag = "2024""#).unwrap();
        assert!(matches!(expr, Expr::And(_, _)));
    }

    #[test]
    fn parse_or_expression() {
        let expr = parse_cql(r#"name ~ "*.jpg" OR name ~ "*.png""#).unwrap();
        assert!(matches!(expr, Expr::Or(_, _)));
    }

    #[test]
    fn parse_operator_precedence() {
        // a OR b AND c => a OR (b AND c)
        let expr = parse_cql(r#"tag = "a" OR tag = "b" AND tag = "c""#).unwrap();
        match expr {
            Expr::Or(left, right) => {
                assert!(matches!(*left, Expr::Comparison { .. }));
                assert!(matches!(*right, Expr::And(_, _)));
            }
            _ => panic!("Expected OR at top level"),
        }
    }

    #[test]
    fn parse_parenthesized_grouping() {
        let expr = parse_cql(r#"(tag = "a" OR tag = "b") AND name ~ "*.jpg""#).unwrap();
        match expr {
            Expr::And(left, right) => {
                assert!(matches!(*left, Expr::Or(_, _)));
                assert!(matches!(*right, Expr::Comparison { .. }));
            }
            _ => panic!("Expected AND at top level"),
        }
    }

    #[test]
    fn parse_not_expression() {
        let expr = parse_cql(r#"NOT tag = "archived""#).unwrap();
        assert!(matches!(expr, Expr::Not(_)));
    }

    #[test]
    fn parse_in_expression() {
        let expr = parse_cql(r#"tag IN ("work", "project")"#).unwrap();
        match &expr {
            Expr::InExpr { field, values } => {
                assert_eq!(*field, Field::Tag);
                assert_eq!(values.len(), 2);
            }
            _ => panic!("Expected InExpr"),
        }
    }

    #[test]
    fn parse_size_literal() {
        let expr = parse_cql("size > 10MB").unwrap();
        match &expr {
            Expr::Comparison {
                value: Value::SizeBytes(bytes),
                ..
            } => {
                assert_eq!(*bytes, 10_485_760);
            }
            _ => panic!("Expected size comparison"),
        }
    }

    #[test]
    fn parse_date_string() {
        let expr = parse_cql(r#"modified > "2024-01-01""#).unwrap();
        match &expr {
            Expr::Comparison {
                value: Value::Timestamp(ts),
                ..
            } => {
                assert_eq!(*ts, 1704067200);
            }
            _ => panic!("Expected date comparison"),
        }
    }

    #[test]
    fn parse_complex_query() {
        let expr =
            parse_cql(r#"(tag = "work" OR tag = "personal") AND name ~ "report*" AND size > 1MB"#);
        assert!(expr.is_ok());
    }

    #[test]
    fn empty_query_returns_error() {
        assert!(matches!(parse_cql(""), Err(CqlParseError::EmptyQuery)));
        assert!(matches!(parse_cql("   "), Err(CqlParseError::EmptyQuery)));
    }

    #[test]
    fn invalid_syntax_returns_error() {
        assert!(parse_cql("tag = ").is_err());
        assert!(parse_cql(r#"= "value""#).is_err());
    }

    #[test]
    fn invalid_operator_for_field() {
        // size does not support ~
        assert!(parse_cql(r#"size ~ "pattern""#).is_err());
        // type does not support >
        assert!(parse_cql(r#"type > "image""#).is_err());
    }

    #[test]
    fn case_insensitive_operators() {
        assert!(parse_cql(r#"tag = "x" and tag = "y""#).is_ok());
        assert!(parse_cql(r#"tag = "x" And tag = "y""#).is_ok());
        assert!(parse_cql(r#"not tag = "x""#).is_ok());
        assert!(parse_cql(r#"tag in ("x", "y")"#).is_ok());
    }

    #[test]
    fn size_units() {
        assert_eq!(parse_size_to_bytes("100B").unwrap(), 100);
        assert_eq!(parse_size_to_bytes("1KB").unwrap(), 1024);
        assert_eq!(parse_size_to_bytes("10MB").unwrap(), 10_485_760);
        assert_eq!(parse_size_to_bytes("1GB").unwrap(), 1_073_741_824);
        assert_eq!(parse_size_to_bytes("1.5MB").unwrap(), 1_572_864);
    }

    #[test]
    fn date_conversion() {
        // 2024-01-01 00:00:00 UTC = 1704067200
        assert_eq!(ymd_to_unix(2024, 1, 1), 1704067200);
        // 1970-01-01 = 0
        assert_eq!(ymd_to_unix(1970, 1, 1), 0);
    }
}
