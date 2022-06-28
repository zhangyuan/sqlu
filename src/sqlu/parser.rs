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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn parse_select_statement_into_json() -> anyhow::Result<()> {
        let ast_as_json = parse_sql_as_json_value("pg", "select id, name from users")?;
        let expected_json = json!(
            {
                "statements": [
                    {
                        "Query": {
                            "body": {
                                "Select": {
                                    "cluster_by": [],
                                    "distinct": false,
                                    "distribute_by": [],
                                    "from": [
                                        {
                                            "joins": [],
                                            "relation": {
                                                "Table": {
                                                    "alias": null,
                                                    "args": null,
                                                    "name": [
                                                        {
                                                            "quote_style": null,
                                                            "value": "users"
                                                        }
                                                    ],
                                                    "with_hints": []
                                                }
                                            }
                                        }
                                    ],
                                    "group_by": [],
                                    "having": null,
                                    "into": null,
                                    "lateral_views": [],
                                    "projection": [
                                        {
                                            "UnnamedExpr": {
                                                "Identifier": {
                                                    "quote_style": null,
                                                    "value": "id"
                                                }
                                            }
                                        },
                                        {
                                            "UnnamedExpr": {
                                                "Identifier": {
                                                    "quote_style": null,
                                                    "value": "name"
                                                }
                                            }
                                        }
                                    ],
                                    "qualify": null,
                                    "selection": null,
                                    "sort_by": [],
                                    "top": null
                                }
                            },
                            "fetch": null,
                            "limit": null,
                            "lock": null,
                            "offset": null,
                            "order_by": [],
                            "with": null
                        }
                    }
                ]
            }
        );

        assert_eq!(expected_json, ast_as_json);
        Ok(())
    }
}
