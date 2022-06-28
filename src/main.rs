use anyhow::Ok;
use serde::{Deserialize, Serialize};
use sqlparser::ast::Statement;
use sqlparser::dialect::PostgreSqlDialect;
use sqlparser::dialect::{Dialect, MySqlDialect, SnowflakeDialect};
use sqlparser::parser::Parser;

use warp::Filter;

#[derive(Deserialize)]
struct ParseRequest {
    sql: String,
    dialect: String,
}

#[derive(Serialize)]
struct Ast {
    statements: Vec<Statement>,
}

#[derive(Serialize)]
struct ParseError {
    message: String,
    sql: String,
}

fn parse_sql(dialect_id: &str, sql: &str) -> anyhow::Result<Ast> {
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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let parse = warp::post()
        .and(warp::path("ast"))
        .and(warp::body::json())
        .map(|request: ParseRequest| {
            let res = parse_sql(&request.dialect, &request.sql)
                .map(|ast| serde_json::to_value(ast).unwrap())
                .or_else(|e| {
                    serde_json::to_value(ParseError {
                        message: e.to_string(),
                        sql: request.sql,
                    })
                });

            warp::reply::json(&res.unwrap())
        });

    warp::serve(parse).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
