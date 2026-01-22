# Query Language Design (JQL-like)

## Target Syntax

```
tag = "vacation" AND tag = "2024"
name ~ "*.jpg" OR name ~ "*.png"
size > 10MB AND modified > "2024-01-01"
tag IN ("work", "project") AND NOT tag = "archived"
```

## Parser Options

| Library | Type | Best For |
|---------|------|----------|
| [pest](https://pest.rs/) | Parser Generator | Complex grammars, readable grammar files |
| [nom](https://github.com/rust-bakery/nom) | Parser Combinator | Performance-critical, fine control |
| [chumsky](https://github.com/zesterer/chumsky) | Parser Combinator | Modern API, good error recovery |

## Pest Grammar Example

```pest
// query.pest
query = { SOI ~ expression ~ EOI }

expression = { term ~ (logical_op ~ term)* }
term = { "(" ~ expression ~ ")" | comparison | function_call }

comparison = { field ~ comparator ~ value }
field = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }
comparator = { "=" | "!=" | "~" | ">" | "<" | ">=" | "<=" }
value = { quoted_string | number | identifier }

logical_op = { "AND" | "OR" | "NOT" }

function_call = { function_name ~ "(" ~ args? ~ ")" }
function_name = { "contains" | "startsWith" | "endsWith" }
args = { value ~ ("," ~ value)* }

quoted_string = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
number = @{ "-"? ~ ASCII_DIGIT+ ~ ("." ~ ASCII_DIGIT+)? }
identifier = @{ ASCII_ALPHA ~ ASCII_ALPHANUMERIC* }

WHITESPACE = _{ " " | "\t" | "\n" }
```

## AST Structure

```rust
pub enum Expr {
    Comparison {
        field: String,
        op: Comparator,
        value: Value,
    },
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    Not(Box<Expr>),
    Group(Box<Expr>),
}

pub enum Comparator {
    Eq,      // =
    NotEq,   // !=
    Like,    // ~
    Gt,      // >
    Lt,      // <
    Gte,     // >=
    Lte,     // <=
    In,      // IN
}

pub enum Value {
    String(String),
    Number(f64),
    List(Vec<Value>),
}
```

## SQL Generation

```rust
impl Expr {
    pub fn to_sql(&self) -> (String, Vec<SqlValue>) {
        match self {
            Expr::Comparison { field, op, value } => {
                let (sql_op, val) = match op {
                    Comparator::Eq => ("=", value.to_sql()),
                    Comparator::Like => ("LIKE", format!("%{}%", value)),
                    // ...
                };
                (format!("{} {} ?", field, sql_op), vec![val])
            }
            Expr::And(left, right) => {
                let (l_sql, l_params) = left.to_sql();
                let (r_sql, r_params) = right.to_sql();
                (
                    format!("({} AND {})", l_sql, r_sql),
                    [l_params, r_params].concat(),
                )
            }
            // ...
        }
    }
}
```

## File Structure

```
src-tauri/src/domain/search/
├── mod.rs
├── query_parser.rs      # pest 解析器
├── query_ast.rs         # AST 結構
└── query_executor.rs    # SQL 生成器
```

## References

- [pest.rs](https://pest.rs/)
- [ANTLR for JQL](https://medium.com/@shriomtripathi33/atlassians-jira-query-language-antlr)
- [Building JQL-like Features](https://yetanotherprogrammingblog.medium.com/how-can-we-create-jiras-jql-like-query-language)
