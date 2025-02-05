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

trait SqlVisitor {
    fn visit_select(
        &mut self,
        columns: &[SqlAst],
        from: &Box<SqlAst>,
        where_clause: &Option<Box<SqlAst>>,
        group_by: &Option<Vec<SqlAst>>,
        order_by: &Option<Vec<SqlAst>>,
    );
    fn visit_column(&mut self, name: &str);
    fn visit_join(
        &mut self,
        left: &SqlAst,
        right: &SqlAst,
        join_type: &JoinType,
        on: &SqlAst,
    );

    fn visit_subquery(&mut self, items: &SqlAst, alias: &str);

    fn visit_where(&mut self, condition: &SqlAst);
    fn visit_list(&mut self, items: &[SqlAst], separator: &str);
    fn visit_table(&mut self, name: &str, alias: &str);
    fn visit_column_alias(&mut self, column: &str, alias: &str);

    fn visit_expression(&mut self, sql_ast: &SqlAst);

    fn visit_condition(
        &mut self,
        left: &SqlAst,
        operator: &Operator,
        right: &SqlAst,
    );
    fn visit_logical(&mut self, items: &[SqlAst], variant: &LogicalVariant);
}

impl SqlAst {
    fn accept<V: SqlVisitor>(&self, visitor: &mut V) {
        match self {
            SqlAst::Select {
                columns,
                from,
                where_clause,
                group_by,
                order_by,
            } => visitor.visit_select(
                columns,
                from,
                where_clause,
                group_by,
                order_by,
            ),
            SqlAst::Table(name, alias) => visitor.visit_table(name, alias), // Assuming reusing visit_column for Table visitation
            SqlAst::Column(name) => visitor.visit_column(name),
            SqlAst::ColumnAlias { column, alias } => {
                visitor.visit_column_alias(column, alias)
            }
            SqlAst::Join {
                left,
                right,
                join_type,
                on,
            } => visitor.visit_join(left, right, join_type, on),
            SqlAst::Expression(sql_ast) => visitor.visit_expression(sql_ast),
            SqlAst::Subquery(sql_ast, alias) => {
                visitor.visit_subquery(sql_ast, alias)
            }

            SqlAst::Logical { items, variant } => {
                visitor.visit_logical(items, variant)
            }
            SqlAst::Comparison {
                left,
                operator,
                right,
            } => visitor.visit_condition(left, operator, right),
        }
    }
}

pub struct SQLGenerator {
    sql: String,
}

impl SQLGenerator {
    pub fn new() -> Self {
        SQLGenerator { sql: String::new() }
    }

    pub fn generate_sql(&mut self, ast: &SqlAst) -> String {
        ast.accept(self);
        self.sql.clone()
    }
}

impl SqlVisitor for SQLGenerator {
    fn visit_select(
        &mut self,
        columns: &[SqlAst],
        from: &Box<SqlAst>,
        where_clause: &Option<Box<SqlAst>>,
        group_by: &Option<Vec<SqlAst>>,
        order_by: &Option<Vec<SqlAst>>,
    ) {
        self.sql.push_str("SELECT");
        self.visit_list(columns, ",");
        self.sql.push_str(" FROM");
        from.accept(self);
        if let Some(where_clause) = where_clause {
            self.sql.push_str(" WHERE");
            self.visit_where(where_clause);
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
    fn visit_column(&mut self, name: &str) {
        self.sql.push_str(&format!(" {}", name));
    }

    fn visit_list(&mut self, items: &[SqlAst], separator: &str) {
        for (index, item) in items.iter().enumerate() {
            if index > 0 {
                self.sql.push_str(separator);
            }
            item.accept(self);
        }
    }
    fn visit_table(&mut self, name: &str, alias: &str) {
        self.sql.push_str(&format!(" {} {}", name, alias));
    }

    fn visit_column_alias(&mut self, column: &str, alias: &str) {
        self.sql.push_str(&format!(" {} AS {}", column, alias));
    }

    fn visit_join(
        &mut self,
        left: &SqlAst,
        right: &SqlAst,
        join_type: &JoinType,
        on: &SqlAst,
    ) {
        left.accept(self);
        let join_str = match join_type {
            JoinType::Inner => " INNER JOIN",
            JoinType::Left => " LEFT JOIN",
            JoinType::Right => " RIGHT JOIN",
            JoinType::Full => " FULL JOIN",
        };
        self.sql.push_str(join_str);
        right.accept(self);
        self.sql.push_str(" ON");
        on.accept(self);
    }

    fn visit_expression(&mut self, sql_ast: &SqlAst) {
        self.sql.push_str(" (");
        sql_ast.accept(self);
        self.sql.push(')');
    }

    fn visit_where(&mut self, condition: &SqlAst) {
        condition.accept(self);
    }

    fn visit_subquery(&mut self, sql_ast: &SqlAst, alias: &str) {
        self.visit_expression(sql_ast);
        self.sql.push_str(&format!(" {}", alias));
    }

    fn visit_condition(
        &mut self,
        left: &SqlAst,
        operator: &Operator,
        right: &SqlAst,
    ) {
        left.accept(self);
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
        if operator == &Operator::In {
            self.visit_expression(right);
        } else {
            right.accept(self);
        }
    }

    fn visit_logical(&mut self, items: &[SqlAst], variant: &LogicalVariant) {
        match variant {
            LogicalVariant::And => self.visit_list(items, " AND"),
            LogicalVariant::Or => {
                self.sql.push_str(" (");
                self.visit_list(items, " OR");
                self.sql.push(')');
            }
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
                panic!("Expected SQLAst::Condition");
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
        let subquery_ast = SqlAst::Expression(Box::new(SqlAst::Select {
            columns: vec![SqlAst::Column(rc!["inner_col"])],
            from: Box::new(SqlAst::Table(
                rc!["inner_table"],
                rc!["inner_table"],
            )),
            where_clause: None,
            group_by: None,
            order_by: None,
        }));

        let inner_join = SqlAst::Join {
            left: Box::new(SqlAst::Table(rc!["table1"], rc!["table1"])),
            right: Box::new(subquery_ast),
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
            "SELECT table1.col1 AS alias1, table2.col2 \
            FROM table1 table1 \
            INNER JOIN (SELECT inner_col FROM inner_table inner_table) ON table1.id = inner_table.fk_id \
            LEFT JOIN table2 table2 ON table1.id = table2.fk_id \
            WHERE date >= ? AND date < ?"
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
