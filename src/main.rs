use anyhow::Ok;
use serde::{Deserialize, Serialize};
use warp::Filter;

use sqlu::sqlu::parser;

#[derive(Deserialize)]
struct ParseRequest {
    sql: String,
    dialect: String,
}

#[derive(Serialize)]
struct ParseError {
    message: String,
    sql: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let parse = warp::post()
        .and(warp::path("ast"))
        .and(warp::body::json())
        .map(|request: ParseRequest| {
            let res =
                parser::parse_sql_as_json_value(&request.dialect, &request.sql).or_else(|e| {
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
