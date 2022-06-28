use serde::{Deserialize, Serialize};
use sqlparser::ast::Statement;
use sqlparser::dialect::SnowflakeDialect;
use sqlparser::parser::Parser;

use warp::Filter;

#[derive(Deserialize)]
struct ParseRequest {
    sql: String,
    dialect: String,
}

#[derive(Serialize)]
struct Ast {
    statements: Vec<Statement>
}

fn parse_sql(_: &str, sql: &str) -> Ast {
    let dialect = SnowflakeDialect {};
    let statements = Parser::parse_sql(&dialect, sql).unwrap();
    Ast { statements } 
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let parse = warp::post()
        .and(warp::path("ast"))
        .and(warp::body::json())
        .map(|request: ParseRequest| {
            let ast = parse_sql(&request.dialect, &request.sql);
            warp::reply::json(&ast)
        });

    warp::serve(parse).run(([127, 0, 0, 1], 3030)).await
}
