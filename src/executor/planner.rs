use crate::domain::models::{Column, Datasource, Filter, ReportRequest};
use crate::executor::query::{JoinType, LogicalVariant, Operator, SqlAst};
use crate::rc;
use std::rc::Rc;

#[derive(Debug)]
pub enum Error {
    ColumnNotFound(String),
    MissingFilter(String),
}

pub struct QueryPlanner {
    datasource: Datasource,
}

impl QueryPlanner {
    pub fn new(datasource: Datasource) -> Self {
        QueryPlanner { datasource }
    }
    pub fn plan(&self, request: ReportRequest) -> Result<SqlAst, Error> {
        let (start_date, end_date) = match request.filters {
            Filter::And { value } => {
                let mut start_date = None;
                let mut end_date = None;
                for f in value.iter() {
                    match f {
                        Filter::Gte { column, value } if column == "date" => {
                            start_date = Some(value.clone())
                        }
                        Filter::Lt { column, value } if column == "date" => {
                            end_date = Some(value.clone())
                        }
                        _ => {}
                    }
                }
                (
                    start_date.ok_or(Error::MissingFilter(
                        "start_date".to_string(),
                    ))?,
                    end_date
                        .ok_or(Error::MissingFilter("end_date".to_string()))?,
                )
            }
            _ => {
                return Err(Error::MissingFilter(
                    "Expected And filter".to_string(),
                ))
            }
        };
        let columns: Vec<Column> = request
            .columns
            .iter()
            .map(|c| self.get_column(c))
            .collect::<Result<Vec<Column>, Error>>()?;

        let aggregation_query = SqlAst::Select {
            columns: columns
                .iter()
                .map(|c| SqlAst::ColumnAlias {
                    column: c.expression.clone(),
                    alias: c.column_id.clone(),
                })
                .collect::<Vec<SqlAst>>(),
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
                        right: Box::new(SqlAst::Literal(start_date)),
                    },
                    SqlAst::Comparison {
                        left: Box::new(SqlAst::Column(rc![
                            "from_unixtime(fact_table.ts, 'YYYY-mm-dd')"
                        ])),
                        operator: Operator::Less,
                        right: Box::new(SqlAst::Literal(end_date)),
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
            columns: vec![],
            from: Box::new(dim_join),
            where_clause: None,
            group_by: None,
            order_by: None,
        };
        Ok(final_query)
    }

    fn get_column(&self, input: &String) -> Result<Column, Error> {
        self.datasource
            .columns
            .iter()
            .find(|c| c.column_id.to_string() == *input)
            .cloned()
            .ok_or(Error::ColumnNotFound(input.clone()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::{
        Column, ColumnType, Datasource, Filter, ReportRequest,
    };

    #[test]
    fn test_plan_success() {
        // Create a dummy Column for testing.
        let column = Column {
            name: std::rc::Rc::from("username"),
            column_id: std::rc::Rc::from("username"),
            expression: std::rc::Rc::from("username"),
            column_type: ColumnType::Grouping,
            data_type: std::rc::Rc::from("text"),
        };

        let datasource = Datasource {
            name: std::rc::Rc::from("default"),
            columns: vec![column],
            // Add other required fields if necessary.
        };

        let planner = QueryPlanner { datasource };

        let request = ReportRequest {
            columns: vec!["username".to_string()],
            filters: Filter::And {
                value: vec![
                    Filter::Gte {
                        column: "date".to_string(),
                        value: "2020-01-01".to_string(),
                    },
                    Filter::Lt {
                        column: "date".to_string(),
                        value: "2021-01-01".to_string(),
                    },
                ],
            },
            sort: vec![],
            // Add other fields if ReportRequest requires them.
        };

        let result = planner.plan(request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_plan_missing_filter() {
        let datasource = Datasource {
            name: std::rc::Rc::from("default"),
            columns: vec![],
            // Add other required fields if necessary.
        };

        let planner = QueryPlanner { datasource };

        let request = ReportRequest {
            columns: vec![],
            filters: Filter::Or { value: vec![] },
            sort: vec![],
            // Add other fields if ReportRequest requires them.
        };

        let result = planner.plan(request);
        assert!(result.is_err());
    }
}
