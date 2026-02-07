//! CQL SQL Executor
//!
//! Converts a parsed CQL AST into SQL WHERE clauses with parameterized values.

use crate::domain::search::ast::{ComparisonOp, Expr, Field, Value};

/// A SQL fragment with its corresponding bound parameters.
pub struct SqlFragment {
    pub sql: String,
    pub params: Vec<rusqlite::types::Value>,
}

/// Converts a CQL expression tree into a SQL WHERE clause.
///
/// The generated SQL references `i` as the items table alias.
/// Tag conditions use EXISTS subqueries with auto-incrementing aliases.
pub fn expr_to_sql(expr: &Expr) -> SqlFragment {
    let mut counter = 0;
    let mut params = Vec::new();
    let sql = build_sql(expr, &mut counter, &mut params);
    SqlFragment { sql, params }
}

fn build_sql(
    expr: &Expr,
    counter: &mut usize,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    match expr {
        Expr::Comparison { field, op, value } => {
            build_comparison_sql(*field, *op, value, counter, params)
        }
        Expr::InExpr { field, values } => build_in_sql(*field, values, counter, params),
        Expr::And(left, right) => {
            let l = build_sql(left, counter, params);
            let r = build_sql(right, counter, params);
            format!("({} AND {})", l, r)
        }
        Expr::Or(left, right) => {
            let l = build_sql(left, counter, params);
            let r = build_sql(right, counter, params);
            format!("({} OR {})", l, r)
        }
        Expr::Not(inner) => {
            let inner_sql = build_sql(inner, counter, params);
            format!("NOT ({})", inner_sql)
        }
    }
}

fn build_comparison_sql(
    field: Field,
    op: ComparisonOp,
    value: &Value,
    counter: &mut usize,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    match field {
        Field::Tag => build_tag_comparison_sql(op, value, counter, params),
        Field::Name => build_name_sql(op, value, params),
        Field::Size => build_size_sql(op, value, params),
        Field::Modified => build_modified_sql(op, value, params),
        Field::Type => build_type_sql(op, value, params),
    }
}

fn build_tag_comparison_sql(
    op: ComparisonOp,
    value: &Value,
    counter: &mut usize,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let idx = *counter;
    *counter += 1;

    let (prefix, condition) = match op {
        ComparisonOp::Eq => {
            let s = extract_string(value);
            params.push(rusqlite::types::Value::Text(s));
            ("EXISTS", format!("t_{}.value = ?", idx))
        }
        ComparisonOp::NotEq => {
            let s = extract_string(value);
            params.push(rusqlite::types::Value::Text(s));
            ("NOT EXISTS", format!("t_{}.value = ?", idx))
        }
        ComparisonOp::Like => {
            let s = extract_string(value);
            let like_pattern = glob_to_like(&s);
            params.push(rusqlite::types::Value::Text(like_pattern));
            ("EXISTS", format!("t_{}.value LIKE ? ESCAPE '\\'", idx))
        }
        _ => unreachable!("Invalid operator for tag field"),
    };

    format!(
        "{} (SELECT 1 FROM item_tags it_{} JOIN tags t_{} ON it_{}.tag_id = t_{}.id \
         WHERE it_{}.item_id = i.id AND {})",
        prefix, idx, idx, idx, idx, idx, condition
    )
}

fn build_tag_in_sql(
    values: &[Value],
    counter: &mut usize,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let idx = *counter;
    *counter += 1;

    let placeholders: Vec<&str> = values.iter().map(|_| "?").collect();
    let placeholders_str = placeholders.join(", ");

    for v in values {
        let s = extract_string(v);
        params.push(rusqlite::types::Value::Text(s));
    }

    format!(
        "EXISTS (SELECT 1 FROM item_tags it_{} JOIN tags t_{} ON it_{}.tag_id = t_{}.id \
         WHERE it_{}.item_id = i.id AND t_{}.value IN ({}))",
        idx, idx, idx, idx, idx, idx, placeholders_str
    )
}

/// SQL expression that extracts the filename from `i.path`.
///
/// Uses SQLite RTRIM trick: strips all non-separator characters from the right,
/// leaving the directory prefix up to the last `\` or `/`. SUBSTR from there gives
/// the filename. Handles both Windows and Unix separators.
const FILENAME_EXPR: &str =
    "LOWER(SUBSTR(i.path, LENGTH(RTRIM(i.path, REPLACE(REPLACE(i.path, '\\', ''), '/', ''))) + 1))";

fn build_name_sql(
    op: ComparisonOp,
    value: &Value,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let s = extract_string(value);
    match op {
        ComparisonOp::Eq => {
            params.push(rusqlite::types::Value::Text(s.to_lowercase()));
            format!("{} = ?", FILENAME_EXPR)
        }
        ComparisonOp::NotEq => {
            params.push(rusqlite::types::Value::Text(s.to_lowercase()));
            format!("{} != ?", FILENAME_EXPR)
        }
        ComparisonOp::Like => {
            let like_pattern = glob_to_like(&s).to_lowercase();
            params.push(rusqlite::types::Value::Text(like_pattern));
            format!("{} LIKE ? ESCAPE '\\'", FILENAME_EXPR)
        }
        _ => unreachable!("Invalid operator for name field"),
    }
}

fn build_size_sql(
    op: ComparisonOp,
    value: &Value,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let bytes = extract_size(value);
    params.push(rusqlite::types::Value::Integer(bytes));
    let sql_op = comparison_op_to_sql(op);
    format!("COALESCE(i.size, 0) {} ?", sql_op)
}

fn build_modified_sql(
    op: ComparisonOp,
    value: &Value,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let ts = extract_timestamp(value);
    params.push(rusqlite::types::Value::Integer(ts));
    let sql_op = comparison_op_to_sql(op);
    format!("COALESCE(i.modified_time, 0) {} ?", sql_op)
}

fn build_type_sql(
    op: ComparisonOp,
    value: &Value,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    let type_name = extract_string(value).to_lowercase();

    if type_name == "directory" {
        return match op {
            ComparisonOp::Eq => "i.is_directory = 1".to_string(),
            ComparisonOp::NotEq => "i.is_directory = 0".to_string(),
            _ => unreachable!("Invalid operator for type field"),
        };
    }

    let extensions = type_to_extensions(&type_name);
    if extensions.is_empty() {
        // Unknown type name — match nothing for =, everything for !=
        return match op {
            ComparisonOp::Eq => "0".to_string(),
            ComparisonOp::NotEq => "1".to_string(),
            _ => unreachable!(),
        };
    }

    let conditions: Vec<String> = extensions
        .iter()
        .map(|ext| {
            params.push(rusqlite::types::Value::Text(format!("%{}", ext)));
            "LOWER(i.path) LIKE ?".to_string()
        })
        .collect();
    let joined = conditions.join(" OR ");

    match op {
        ComparisonOp::Eq => format!("(i.is_directory = 0 AND ({}))", joined),
        ComparisonOp::NotEq => format!("(i.is_directory = 1 OR NOT ({}))", joined),
        _ => unreachable!("Invalid operator for type field"),
    }
}

fn build_in_sql(
    field: Field,
    values: &[Value],
    counter: &mut usize,
    params: &mut Vec<rusqlite::types::Value>,
) -> String {
    match field {
        Field::Tag => build_tag_in_sql(values, counter, params),
        Field::Name => {
            let placeholders: Vec<&str> = values.iter().map(|_| "?").collect();
            for v in values {
                let s = extract_string(v);
                params.push(rusqlite::types::Value::Text(s.to_lowercase()));
            }
            format!("{} IN ({})", FILENAME_EXPR, placeholders.join(", "))
        }
        Field::Type => {
            let mut all_conditions = Vec::new();
            for v in values {
                let type_name = extract_string(v).to_lowercase();
                if type_name == "directory" {
                    all_conditions.push("i.is_directory = 1".to_string());
                } else {
                    let extensions = type_to_extensions(&type_name);
                    for ext in extensions {
                        params.push(rusqlite::types::Value::Text(format!("%{}", ext)));
                        all_conditions.push("LOWER(i.path) LIKE ?".to_string());
                    }
                }
            }
            if all_conditions.is_empty() {
                "0".to_string()
            } else {
                format!("({})", all_conditions.join(" OR "))
            }
        }
        _ => unreachable!("IN not supported for this field"),
    }
}

/// Converts a glob pattern to SQL LIKE pattern.
///
/// `*` → `%`, `?` → `_`, literal `%` and `_` are escaped with `\`.
fn glob_to_like(glob: &str) -> String {
    let mut result = String::with_capacity(glob.len());
    for ch in glob.chars() {
        match ch {
            '*' => result.push('%'),
            '?' => result.push('_'),
            '%' => {
                result.push('\\');
                result.push('%');
            }
            '_' => {
                result.push('\\');
                result.push('_');
            }
            '\\' => {
                result.push('\\');
                result.push('\\');
            }
            _ => result.push(ch),
        }
    }
    result
}

/// Maps a type name to file extensions (matching frontend FilterOptionsPanel).
fn type_to_extensions(type_name: &str) -> &'static [&'static str] {
    match type_name {
        "image" => &[
            ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".ico", ".tiff", ".tif",
        ],
        "video" => &[".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv", ".webm", ".m4v"],
        "document" => &[
            ".pdf", ".doc", ".docx", ".xls", ".xlsx", ".ppt", ".pptx", ".txt", ".csv", ".rtf",
        ],
        "audio" => &[".mp3", ".wav", ".flac", ".aac", ".ogg", ".wma", ".m4a"],
        "archive" => &[".zip", ".rar", ".7z", ".tar", ".gz", ".bz2", ".xz"],
        _ => &[],
    }
}

fn comparison_op_to_sql(op: ComparisonOp) -> &'static str {
    match op {
        ComparisonOp::Eq => "=",
        ComparisonOp::NotEq => "!=",
        ComparisonOp::Gt => ">",
        ComparisonOp::Lt => "<",
        ComparisonOp::Gte => ">=",
        ComparisonOp::Lte => "<=",
        ComparisonOp::Like => unreachable!("LIKE handled separately"),
    }
}

fn extract_string(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => unreachable!("Expected string value"),
    }
}

fn extract_size(value: &Value) -> i64 {
    match value {
        Value::SizeBytes(bytes) => *bytes,
        Value::Number(n) => *n as i64,
        _ => unreachable!("Expected size value"),
    }
}

fn extract_timestamp(value: &Value) -> i64 {
    match value {
        Value::Timestamp(ts) => *ts,
        Value::Number(n) => *n as i64,
        _ => unreachable!("Expected timestamp value"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_eq() {
        let expr = Expr::Comparison {
            field: Field::Tag,
            op: ComparisonOp::Eq,
            value: Value::String("vacation".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("EXISTS"));
        assert!(frag.sql.contains("t_0.value = ?"));
        assert_eq!(frag.params.len(), 1);
    }

    #[test]
    fn tag_neq() {
        let expr = Expr::Comparison {
            field: Field::Tag,
            op: ComparisonOp::NotEq,
            value: Value::String("archived".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("NOT EXISTS"));
        assert_eq!(frag.params.len(), 1);
    }

    #[test]
    fn tag_like() {
        let expr = Expr::Comparison {
            field: Field::Tag,
            op: ComparisonOp::Like,
            value: Value::String("vac*".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("LIKE ? ESCAPE"));
        match &frag.params[0] {
            rusqlite::types::Value::Text(s) => assert_eq!(s, "vac%"),
            _ => panic!("Expected text param"),
        }
    }

    #[test]
    fn tag_in() {
        let expr = Expr::InExpr {
            field: Field::Tag,
            values: vec![
                Value::String("work".to_string()),
                Value::String("project".to_string()),
            ],
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("t_0.value IN (?, ?)"));
        assert_eq!(frag.params.len(), 2);
    }

    #[test]
    fn name_like_glob() {
        let expr = Expr::Comparison {
            field: Field::Name,
            op: ComparisonOp::Like,
            value: Value::String("*.jpg".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains(FILENAME_EXPR));
        assert!(frag.sql.contains("LIKE ? ESCAPE"));
        match &frag.params[0] {
            rusqlite::types::Value::Text(s) => assert_eq!(s, "%.jpg"),
            _ => panic!("Expected text param"),
        }
    }

    #[test]
    fn size_gt() {
        let expr = Expr::Comparison {
            field: Field::Size,
            op: ComparisonOp::Gt,
            value: Value::SizeBytes(10_485_760),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("COALESCE(i.size, 0) > ?"));
        match &frag.params[0] {
            rusqlite::types::Value::Integer(n) => assert_eq!(*n, 10_485_760),
            _ => panic!("Expected integer param"),
        }
    }

    #[test]
    fn modified_gt() {
        let expr = Expr::Comparison {
            field: Field::Modified,
            op: ComparisonOp::Gt,
            value: Value::Timestamp(1704067200),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("COALESCE(i.modified_time, 0) > ?"));
    }

    #[test]
    fn type_image() {
        let expr = Expr::Comparison {
            field: Field::Type,
            op: ComparisonOp::Eq,
            value: Value::String("image".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("i.is_directory = 0"));
        assert!(frag.sql.contains("LOWER(i.path) LIKE ?"));
        // Should have one param per extension
        assert_eq!(frag.params.len(), 10); // 10 image extensions
    }

    #[test]
    fn type_directory() {
        let expr = Expr::Comparison {
            field: Field::Type,
            op: ComparisonOp::Eq,
            value: Value::String("directory".to_string()),
        };
        let frag = expr_to_sql(&expr);
        assert_eq!(frag.sql, "i.is_directory = 1");
        assert_eq!(frag.params.len(), 0);
    }

    #[test]
    fn and_expression() {
        let expr = Expr::And(
            Box::new(Expr::Comparison {
                field: Field::Tag,
                op: ComparisonOp::Eq,
                value: Value::String("a".to_string()),
            }),
            Box::new(Expr::Comparison {
                field: Field::Tag,
                op: ComparisonOp::Eq,
                value: Value::String("b".to_string()),
            }),
        );
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains(" AND "));
        assert!(frag.sql.contains("t_0.value = ?"));
        assert!(frag.sql.contains("t_1.value = ?"));
        assert_eq!(frag.params.len(), 2);
    }

    #[test]
    fn not_tag() {
        let expr = Expr::Not(Box::new(Expr::Comparison {
            field: Field::Tag,
            op: ComparisonOp::Eq,
            value: Value::String("archived".to_string()),
        }));
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("NOT (EXISTS"));
    }

    #[test]
    fn glob_to_like_conversion() {
        assert_eq!(glob_to_like("*.jpg"), "%.jpg");
        assert_eq!(glob_to_like("report?"), "report_");
        assert_eq!(glob_to_like("100%"), "100\\%");
        assert_eq!(glob_to_like("a_b"), "a\\_b");
    }

    #[test]
    fn complex_query() {
        // (tag = "a" OR tag = "b") AND size > 5MB
        let expr = Expr::And(
            Box::new(Expr::Or(
                Box::new(Expr::Comparison {
                    field: Field::Tag,
                    op: ComparisonOp::Eq,
                    value: Value::String("a".to_string()),
                }),
                Box::new(Expr::Comparison {
                    field: Field::Tag,
                    op: ComparisonOp::Eq,
                    value: Value::String("b".to_string()),
                }),
            )),
            Box::new(Expr::Comparison {
                field: Field::Size,
                op: ComparisonOp::Gt,
                value: Value::SizeBytes(5_242_880),
            }),
        );
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains(" OR "));
        assert!(frag.sql.contains(" AND "));
        assert!(frag.sql.contains("COALESCE(i.size, 0) > ?"));
        assert_eq!(frag.params.len(), 3);
    }

    #[test]
    fn counter_increments_for_each_tag_subquery() {
        let expr = Expr::And(
            Box::new(Expr::Comparison {
                field: Field::Tag,
                op: ComparisonOp::Eq,
                value: Value::String("a".to_string()),
            }),
            Box::new(Expr::And(
                Box::new(Expr::Comparison {
                    field: Field::Tag,
                    op: ComparisonOp::Eq,
                    value: Value::String("b".to_string()),
                }),
                Box::new(Expr::InExpr {
                    field: Field::Tag,
                    values: vec![
                        Value::String("c".to_string()),
                        Value::String("d".to_string()),
                    ],
                }),
            )),
        );
        let frag = expr_to_sql(&expr);
        assert!(frag.sql.contains("it_0"));
        assert!(frag.sql.contains("it_1"));
        assert!(frag.sql.contains("it_2"));
    }
}
