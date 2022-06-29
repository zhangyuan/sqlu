# sqlparser

A simple HTTP API wrapping [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).

## Example

Request

POST `/ast/`

```json
{
    "dialect": "snowflake",
    "sql": "with source as ( select id, name, count1, count2 from users) select name, max(count1, count2) as max_count from source"
}
```

Response

```json
[{
    "Query": {
        "body": {
            "Select": {
                "cluster_by": [],
                "distinct": false,
                "distribute_by": [],
                "from": [{
                    "joins": [],
                    "relation": {
                        "Table": {
                            "alias": null,
                            "args": null,
                            "name": [{
                                "quote_style": null,
                                "value": "source"
                            }],
                            "with_hints": []
                        }
                    }
                }],
                "group_by": [],
                "having": null,
                "into": null,
                "lateral_views": [],
                "projection": [{
                        "UnnamedExpr": {
                            "Identifier": {
                                "quote_style": null,
                                "value": "name"
                            }
                        }
                    },
                    {
                        "ExprWithAlias": {
                            "alias": {
                                "quote_style": null,
                                "value": "max_count"
                            },
                            "expr": {
                                "Function": {
                                    "args": [{
                                            "Unnamed": {
                                                "Expr": {
                                                    "Identifier": {
                                                        "quote_style": null,
                                                        "value": "count1"
                                                    }
                                                }
                                            }
                                        },
                                        {
                                            "Unnamed": {
                                                "Expr": {
                                                    "Identifier": {
                                                        "quote_style": null,
                                                        "value": "count2"
                                                    }
                                                }
                                            }
                                        }
                                    ],
                                    "distinct": false,
                                    "name": [{
                                        "quote_style": null,
                                        "value": "max"
                                    }],
                                    "over": null
                                }
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
        "with": {
            "cte_tables": [{
                "alias": {
                    "columns": [],
                    "name": {
                        "quote_style": null,
                        "value": "source"
                    }
                },
                "from": null,
                "query": {
                    "body": {
                        "Select": {
                            "cluster_by": [],
                            "distinct": false,
                            "distribute_by": [],
                            "from": [{
                                "joins": [],
                                "relation": {
                                    "Table": {
                                        "alias": null,
                                        "args": null,
                                        "name": [{
                                            "quote_style": null,
                                            "value": "users"
                                        }],
                                        "with_hints": []
                                    }
                                }
                            }],
                            "group_by": [],
                            "having": null,
                            "into": null,
                            "lateral_views": [],
                            "projection": [{
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
                                },
                                {
                                    "UnnamedExpr": {
                                        "Identifier": {
                                            "quote_style": null,
                                            "value": "count1"
                                        }
                                    }
                                },
                                {
                                    "UnnamedExpr": {
                                        "Identifier": {
                                            "quote_style": null,
                                            "value": "count2"
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
            }],
            "recursive": false
        }
    }
}]
```
