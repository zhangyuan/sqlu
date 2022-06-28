# sqlparser

A simple HTTP API wrapping [sqlparser-rs](https://github.com/sqlparser-rs/sqlparser-rs).

## Example

Request

POST `/ast/`

```json
{
    "dialect": "pg",
    "sql": "with source as ( select id, name from users) select name, count(*) from source group by name"
}
```

Response

```json
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
                                                "value": "source"
                                            }
                                        ],
                                        "with_hints": []
                                    }
                                }
                            }
                        ],
                        "group_by": [
                            {
                                "Identifier": {
                                    "quote_style": null,
                                    "value": "name"
                                }
                            }
                        ],
                        "having": null,
                        "into": null,
                        "lateral_views": [],
                        "projection": [
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
                                    "Function": {
                                        "args": [
                                            {
                                                "Unnamed": "Wildcard"
                                            }
                                        ],
                                        "distinct": false,
                                        "name": [
                                            {
                                                "quote_style": null,
                                                "value": "count"
                                            }
                                        ],
                                        "over": null
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
                    "cte_tables": [
                        {
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
                    ],
                    "recursive": false
                }
            }
        }
    ]
}
```
