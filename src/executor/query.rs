use std::rc::Rc;

pub enum SqlAst {
    Select {
        columns: Vec<SqlAst>,
        from: Box<SqlAst>,
        where_clause: Option<Box<SqlAst>>,
        group_by: Option<Vec<SqlAst>>,
        order_by: Option<Vec<SqlAst>>,
    },
    Table(Rc<str>, Rc<str>),
    Subquery(Box<SqlAst>, Rc<str>),
    Column(Rc<str>),
    ColumnAlias {
        column: Rc<str>,
        alias: Rc<str>,
    },
    Join {
        left: Box<SqlAst>,
        right: Box<SqlAst>,
        join_type: JoinType,
        on: Box<SqlAst>,
    },
    Expression(Box<SqlAst>),
    Literal(String),
    Comparison {
        left: Box<SqlAst>,
        operator: Operator,
        right: Box<SqlAst>,
    },
    Logical {
        items: Vec<SqlAst>,
        variant: LogicalVariant,
    },
}

#[derive(Debug, PartialEq)]
pub enum JoinType {
    Inner,
    Left,
    Right,
    Full,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal,
    NotEqual,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    In,
}
#[derive(Debug, PartialEq)]
pub enum LogicalVariant {
    And,
    Or,
}

pub struct SQLGenerator {
    sql: String,
}

impl SQLGenerator {
    pub fn new() -> Self {
        SQLGenerator { sql: String::new() }
    }

    pub fn generate_sql(&mut self, ast: &SqlAst) -> String {
        self.visit(ast);
        self.sql.clone()
    }

    fn visit(&mut self, ast: &SqlAst) {
        match ast {
            SqlAst::Select {
                columns,
                from,
                where_clause,
                group_by,
                order_by,
            } => {
                self.sql.push_str("SELECT");
                self.visit_list(columns, ",");
                self.sql.push_str(" FROM");
                self.visit(from);
                if let Some(where_clause) = where_clause {
                    self.sql.push_str(" WHERE");
                    self.visit(where_clause);
                }
                if let Some(group_by_clause) = group_by {
                    self.sql.push_str(" GROUP BY");
                    self.visit_list(group_by_clause, ",");
                }
                if let Some(order_by_clause) = order_by {
                    self.sql.push_str(" ORDER BY");
                    self.visit_list(order_by_clause, ",");
                }
            }
            SqlAst::Table(name, alias) => {
                self.sql.push_str(&format!(" {} {}", name, alias));
            }
            SqlAst::Column(name) => {
                self.sql.push_str(&format!(" {}", name));
            }
            SqlAst::ColumnAlias { column, alias } => {
                self.sql.push_str(&format!(" {} AS {}", column, alias));
            }
            SqlAst::Join {
                left,
                right,
                join_type,
                on,
            } => {
                self.visit(left);
                let join_str = match join_type {
                    JoinType::Inner => " INNER JOIN",
                    JoinType::Left => " LEFT JOIN",
                    JoinType::Right => " RIGHT JOIN",
                    JoinType::Full => " FULL JOIN",
                };
                self.sql.push_str(join_str);
                self.visit(right);
                self.sql.push_str(" ON");
                self.visit(on);
            }
            SqlAst::Expression(sql_ast) => {
                self.sql.push_str(" (");
                self.visit(sql_ast);
                self.sql.push(')');
            }
            SqlAst::Subquery(sql_ast, alias) => {
                self.sql.push_str(" (");
                self.visit(sql_ast);
                self.sql.push_str(&format!(") {}", alias));
            }
            SqlAst::Logical { items, variant } => match variant {
                LogicalVariant::And => self.visit_list(items, " AND"),
                LogicalVariant::Or => {
                    self.sql.push_str(" (");
                    self.visit_list(items, " OR");
                    self.sql.push(')');
                }
            },
            SqlAst::Comparison {
                left,
                operator,
                right,
            } => {
                self.visit(left);
                let operator_str = match operator {
                    Operator::Equal => " =",
                    Operator::NotEqual => " <>",
                    Operator::Less => " <",
                    Operator::Greater => " >",
                    Operator::LessOrEqual => " <=",
                    Operator::GreaterOrEqual => " >=",
                    Operator::In => " IN",
                };
                self.sql.push_str(operator_str);
                if let Operator::In = operator {
                    self.sql.push_str(" (");
                    self.visit(right);
                    self.sql.push(')');
                } else {
                    self.visit(right);
                }
            }
            SqlAst::Literal(value) => {
                self.sql.push_str(&format!(" {}", value));
            }
        }
    }

    fn visit_list(&mut self, items: &[SqlAst], separator: &str) {
        for (index, item) in items.iter().enumerate() {
            if index > 0 {
                self.sql.push_str(separator);
            }
            self.visit(item);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc;

    #[test]
    fn test_sql_ast() {
        let column = SqlAst::Column(rc!["username"]);
        let column_alias = SqlAst::ColumnAlias {
            column: rc!["username"],
            alias: rc!["user"],
        };

        let join_clause = SqlAst::Join {
            left: Box::new(SqlAst::Table(rc!["orders"], rc!["orders"])),
            right: Box::new(SqlAst::Table(rc!["users"], rc!["users"])),
            join_type: JoinType::Inner,
            on: Box::new(SqlAst::Comparison {
                left: Box::new(SqlAst::Column(rc!["orders.user_id"])),
                operator: Operator::Equal,
                right: Box::new(SqlAst::Column(rc!["users.id"])),
            }),
        };

        if let SqlAst::Column(name) = column {
            assert_eq!(name.as_ref(), "username");
        } else {
            panic!("Expected SQLAst::Column");
        }

        if let SqlAst::ColumnAlias { column, alias } = column_alias {
            assert_eq!(column, rc!["username"]);
            assert_eq!(alias, rc!["user"]);
        } else {
            panic!("Expected SQLAst::ColumnAlias");
        }

        if let SqlAst::Join {
            left,
            right,
            join_type,
            on,
        } = join_clause
        {
            if let SqlAst::Table(left_name, alias) = *left {
                assert_eq!(left_name, rc!["orders"]);
                assert_eq!(alias, rc!["orders"]);
            } else {
                panic!("Expected SQLAst::Table");
            }
            if let SqlAst::Table(right_name, alias) = *right {
                assert_eq!(right_name, rc!["users"]);
                assert_eq!(alias, rc!["users"]);
            } else {
                panic!("Expected SQLAst::Table");
            }
            assert_eq!(join_type, JoinType::Inner);

            if let SqlAst::Comparison {
                left: on_left,
                operator,
                right: on_right,
            } = *on
            {
                assert_eq!(operator, Operator::Equal);
                if let SqlAst::Column(on_left_name) = *on_left {
                    assert_eq!(on_left_name, rc!["orders.user_id"]);
                } else {
                    panic!("Expected SQLAst::Column");
                }
                if let SqlAst::Column(on_right_name) = *on_right {
                    assert_eq!(on_right_name, rc!["users.id"]);
                } else {
                    panic!("Expected SQLAst::Column");
                }
            } else {
                panic!("Expected SQLAst::Comparison");
            }
        } else {
            panic!("Expected SQLAst::Join");
        }
    }

    #[test]
    fn test_generate_sql() {
        let final_query = SqlAst::Select {
            columns: vec![
                SqlAst::ColumnAlias {
                    column: rc!["username"],
                    alias: rc!["user"],
                },
                SqlAst::Column(rc!["email"]),
            ],
            from: Box::new(SqlAst::Table(rc!["users"], rc!["users"])),
            where_clause: Some(Box::new(SqlAst::Comparison {
                left: Box::new(SqlAst::Column(rc!["age"])),
                operator: Operator::GreaterOrEqual,
                right: Box::new(SqlAst::Column(rc!["18"])),
            })),
            group_by: None,
            order_by: Some(vec![SqlAst::Column(rc!["username"])]),
        };

        let mut generator = SQLGenerator::new();
        let sql = generator.generate_sql(&final_query);

        assert_eq!(
            sql.trim(),
            "SELECT username AS user, email FROM users users WHERE age >= 18 ORDER BY username"
        );
    }

    #[test]
    fn test_generate_sql_with_subquery_and_joins() {
        let subquery_ast = SqlAst::Select {
            columns: vec![SqlAst::Column(rc!["inner_col"])],
            from: Box::new(SqlAst::Table(
                rc!["inner_table"],
                rc!["inner_table"],
            )),
            where_clause: None,
            group_by: None,
            order_by: None,
        };

        let inner_join = SqlAst::Join {
            left: Box::new(SqlAst::Table(rc!["table1"], rc!["table1"])),
            right: Box::new(SqlAst::Expression(Box::new(subquery_ast))),
            join_type: JoinType::Inner,
            on: Box::new(SqlAst::Comparison {
                left: Box::new(SqlAst::Column(rc!["table1.id"])),
                operator: Operator::Equal,
                right: Box::new(SqlAst::Column(rc!["inner_table.fk_id"])),
            }),
        };

        let left_join = SqlAst::Join {
            left: Box::new(inner_join),
            right: Box::new(SqlAst::Table(rc!["table2"], rc!["table2"])),
            join_type: JoinType::Left,
            on: Box::new(SqlAst::Comparison {
                left: Box::new(SqlAst::Column(rc!["table1.id"])),
                operator: Operator::Equal,
                right: Box::new(SqlAst::Column(rc!["table2.fk_id"])),
            }),
        };

        let final_query = SqlAst::Select {
            columns: vec![
                SqlAst::ColumnAlias {
                    column: rc!["table1.col1"],
                    alias: rc!["alias1"],
                },
                SqlAst::Column(rc!["table2.col2"]),
            ],
            from: Box::new(left_join),
            where_clause: Some(Box::new(SqlAst::Logical {
                items: vec![
                    SqlAst::Comparison {
                        left: Box::new(SqlAst::Column(rc!["date"])),
                        operator: Operator::GreaterOrEqual,
                        right: Box::new(SqlAst::Column(rc!["?"])),
                    },
                    SqlAst::Comparison {
                        left: Box::new(SqlAst::Column(rc!["date"])),
                        operator: Operator::Less,
                        right: Box::new(SqlAst::Column(rc!["?"])),
                    },
                ],
                variant: LogicalVariant::And,
            })),
            group_by: None,
            order_by: None,
        };

        let mut generator = SQLGenerator::new();
        let sql = generator.generate_sql(&final_query);

        assert_eq!(
            sql.trim(),
            "SELECT table1.col1 AS alias1, table2.col2 FROM table1 table1 INNER JOIN (SELECT inner_col FROM inner_table inner_table) ON table1.id = inner_table.fk_id LEFT JOIN table2 table2 ON table1.id = table2.fk_id WHERE date >= ? AND date < ?"
        );
    }

    #[test]
    fn test_generate_sql_report() {
        let aggregation_query = SqlAst::Select {
            columns: vec![
                SqlAst::ColumnAlias {
                    column: rc!["from_unixtime(fact_table.ts, 'YYYY-mm-dd')"],
                    alias: rc!["date"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["campaign_hierarchy.campaign_id"],
                    alias: rc!["campaign_id"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["fact_table.line_item_id"],
                    alias: rc!["line_item_id"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["sum(fact_table.impressions)"],
                    alias: rc!["sum_impressions"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["sum(fact_table.clicks)"],
                    alias: rc!["sum_clicks"],
                },
            ],
            from: Box::new(SqlAst::Join {
                left: Box::new(SqlAst::Table(
                    rc!["fact_table"],
                    rc!["fact_table"],
                )),
                right: Box::new(SqlAst::Table(
                    rc!["campaign_hierarchy"],
                    rc!["campaign_hierarchy"],
                )),
                join_type: JoinType::Left,
                on: Box::new(SqlAst::Comparison {
                    left: Box::new(SqlAst::Column(rc![
                        "fact_table.line_item_id"
                    ])),
                    operator: Operator::Equal,
                    right: Box::new(SqlAst::Column(rc![
                        "campaign_hierarchy.line_item_id"
                    ])),
                }),
            }),
            where_clause: Some(Box::new(SqlAst::Logical {
                items: vec![
                    SqlAst::Comparison {
                        left: Box::new(SqlAst::Column(rc![
                            "from_unixtime(fact_table.ts, 'YYYY-mm-dd')"
                        ])),
                        operator: Operator::GreaterOrEqual,
                        right: Box::new(SqlAst::Column(rc!["?"])),
                    },
                    SqlAst::Comparison {
                        left: Box::new(SqlAst::Column(rc![
                            "from_unixtime(fact_table.ts, 'YYYY-mm-dd')"
                        ])),
                        operator: Operator::Less,
                        right: Box::new(SqlAst::Column(rc!["?"])),
                    },
                ],
                variant: LogicalVariant::And,
            })),
            group_by: Some(vec![
                SqlAst::Column(rc![
                    "from_unixtime(fact_table.ts, 'YYYY-mm-dd')"
                ]),
                SqlAst::Column(rc!["fact_table.line_item_id"]),
                SqlAst::Column(rc!["campaign_hierarchy.campaign_id"]),
            ]),
            order_by: None,
        };

        let dim_join = SqlAst::Join {
            left: Box::new(SqlAst::Subquery(
                Box::new(aggregation_query),
                rc!["facts"],
            )),
            right: Box::new(SqlAst::Table(
                rc!["dim_campaign"],
                rc!["dim_campaign"],
            )),
            join_type: JoinType::Left,
            on: Box::new(SqlAst::Comparison {
                left: Box::new(SqlAst::Column(rc!["facts.campaign_id"])),
                operator: Operator::Equal,
                right: Box::new(SqlAst::Column(rc![
                    "dim_campaign.campaign_id"
                ])),
            }),
        };

        let final_query = SqlAst::Select {
            columns: vec![
                SqlAst::ColumnAlias {
                    column: rc!["facts.date"],
                    alias: rc!["date"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["facts.campaign_id"],
                    alias: rc!["campaign_id"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["dim_campaign.campaign_name"],
                    alias: rc!["campaign_name"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["facts.line_item_id"],
                    alias: rc!["line_item_id"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["dim_campaign.line_item_name"],
                    alias: rc!["line_item_name"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["facts.sum_impressions"],
                    alias: rc!["sum_impressions"],
                },
                SqlAst::ColumnAlias {
                    column: rc!["facts.sum_clicks"],
                    alias: rc!["sum_clicks"],
                },
            ],
            from: Box::new(dim_join),
            where_clause: None,
            group_by: None,
            order_by: None,
        };

        let mut generator = SQLGenerator::new();
        let sql = generator.generate_sql(&final_query);

        assert_eq!(
            sql.trim(),
            "SELECT facts.date AS date, facts.campaign_id AS campaign_id, dim_campaign.campaign_name AS campaign_name, facts.line_item_id AS line_item_id, dim_campaign.line_item_name AS line_item_name, facts.sum_impressions AS sum_impressions, facts.sum_clicks AS sum_clicks FROM (SELECT from_unixtime(fact_table.ts, 'YYYY-mm-dd') AS date, campaign_hierarchy.campaign_id AS campaign_id, fact_table.line_item_id AS line_item_id, sum(fact_table.impressions) AS sum_impressions, sum(fact_table.clicks) AS sum_clicks FROM fact_table fact_table LEFT JOIN campaign_hierarchy campaign_hierarchy ON fact_table.line_item_id = campaign_hierarchy.line_item_id WHERE from_unixtime(fact_table.ts, 'YYYY-mm-dd') >= ? AND from_unixtime(fact_table.ts, 'YYYY-mm-dd') < ? GROUP BY from_unixtime(fact_table.ts, 'YYYY-mm-dd'), fact_table.line_item_id, campaign_hierarchy.campaign_id) facts LEFT JOIN dim_campaign dim_campaign ON facts.campaign_id = dim_campaign.campaign_id"
        );
    }
}
