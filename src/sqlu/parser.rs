use anyhow::Ok;
use serde::Serialize;
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::dialect::{Dialect, MySqlDialect, SnowflakeDialect};
use sqlparser::parser::Parser;

#[derive(Serialize)]
pub struct Ast {
    statements: Vec<Statement>,
}

pub fn parse_sql(dialect_id: &str, sql: &str) -> anyhow::Result<Ast> {
    let dialect: anyhow::Result<Box<dyn Dialect>> = match dialect_id {
        "snowflake" | "sf" => Ok(Box::new(SnowflakeDialect {})),
        "postgres" | "pg" => Ok(Box::new(PostgreSqlDialect {})),
        "mysql" => Ok(Box::new(MySqlDialect {})),
        _ => Err(anyhow::anyhow!("invalid dialect")),
    };

    let dialect = dialect?;

    let statements = Parser::parse_sql(dialect.as_ref(), sql)?;
    let ast = Ast { statements };
    Ok(ast)
}

pub fn parse_sql_as_json_value(dialect_id: &str, sql: &str) -> anyhow::Result<serde_json::Value> {
    parse_sql(dialect_id, sql).map(|ast| serde_json::to_value(ast).unwrap())
}
