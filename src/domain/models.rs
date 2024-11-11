use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Serialize, Deserialize)]
pub struct Datasource {
    pub name: Rc<str>,
    pub columns: Vec<Column>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Column {
    pub name: Rc<str>,
    pub column_id: Rc<str>,
    pub expression: Rc<str>,
    pub data_type: Rc<str>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum ReportStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Expired,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Filter {
    And { value: Vec<Filter> },
    Or { value: Vec<Filter> },
    Eq { column: String, value: String },
    Lt { column: String, value: String },
    Lte { column: String, value: String },
    Gt { column: String, value: String },
    Gte { column: String, value: String },
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "dir", rename_all = "snake_case")]
pub enum Order {
    Asc { column: String },
    Desc { column: String },
}

#[derive(Serialize, Deserialize)]
pub struct ReportRequest {
    pub columns: Vec<String>,
    pub filters: Filter,
    pub sort: Vec<Order>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ReportMetadata {
    created_at: u64,
    updated_at: u64,
    expires_at: u64,
    num_rows: u64,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Report {
    pub id: Rc<str>,
    pub columns: Vec<Column>,
    pub status: ReportStatus,
    pub metadata: Option<ReportMetadata>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::{load_json, load_yaml};

    #[test]
    fn test_deserialize_request() {
        let request_file = "test/report_request.json";
        let request: ReportRequest =
            load_json(request_file).expect("Could not parse request json");
        if let Filter::And { value } = request.filters {
            value.iter().for_each(|f| match f {
                Filter::Gte { column, value } => {
                    assert_eq!(column, "date");
                    assert_eq!(value, "2020-01-01");
                }
                Filter::Lt { column, value } => {
                    assert_eq!(column, "date");
                    assert_eq!(value, "2021-01-01");
                }
                _ => unreachable!(),
            })
        } else {
            unreachable!();
        }
    }

    #[test]
    fn test_deserialize_datasource() {
        let datasource_file = "test/datasource.yaml";
        let datasource: Datasource =
            load_yaml(datasource_file).expect("Could not parse request yaml");
        assert_eq!(datasource.name, Rc::from("default"));
        datasource
            .columns
            .iter()
            .for_each(|c| match c.column_id.as_ref() {
                "date" => {
                    assert_eq!(c.name.to_string(), "T_DATE".to_string())
                }
                "campaign_id" => {
                    assert_eq!(
                        c.expression.to_string(),
                        "insertion_order.campaign_id".to_string()
                    )
                }
                "insertion_order_id" => {
                    assert_eq!(c.data_type.to_string(), "i32".to_string())
                }
                "line_item_id" => {
                    assert_eq!(c.name.to_string(), "T_LINE_ITEM_ID".to_string())
                }
                "sum_impressions" => {
                    assert_eq!(
                        c.expression.to_string(),
                        "sum(impressions)".to_string()
                    )
                }
                "sum_spend" => {
                    assert_eq!(c.data_type.to_string(), "dec64".to_string())
                }
                _ => unreachable!(),
            })
    }
}
