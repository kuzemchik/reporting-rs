use crate::common::{load_json, load_yaml};
use reporting::domain::models::{
    Column, ColumnType, Datasource, ReportRequest, ReportStatus,
};
use reporting::domain::service::ReportService;
use std::rc::Rc;

mod common;
#[test]
fn test_report_conversion() {
    // let request_string = .unwrap();
    let request_file = "test/report_request.json";
    let request: ReportRequest =
        load_json(request_file).expect("Could not parse request json");

    let datasource_file = "test/datasource.yaml";
    let datasource: Datasource =
        load_yaml(datasource_file).expect("Could not parse request yaml");

    let report_service = ReportService::new(datasource);
    let report = report_service.create_report(request);

    assert_eq!(report.status, ReportStatus::Pending);
}
#[test]
fn integration_test_generated_query() {
    use reporting::executor::planner::QueryPlanner;
    use reporting::domain::models::{Datasource, Column, ReportRequest, Filter, ColumnType};
    use reporting::executor::query::SQLGenerator;
    use reporting::rc;

    let column = Column {
        name: rc!["username"],
        column_id: rc!["username"],
        expression: rc!["username"],
        column_type: ColumnType::Grouping,
        data_type: rc!["text"],
    };

    let datasource = Datasource {
        name: rc!["default"],
        columns: vec![column],
    };

    let planner = QueryPlanner::new(datasource);
    let request = ReportRequest {
        columns: vec!["username".to_string()],
        filters: Filter::And { value: vec![
            Filter::Gte { column: "date".to_string(), value: "2020-01-01".to_string() },
            Filter::Lt { column: "date".to_string(), value: "2021-01-01".to_string() },
        ]},
        sort: vec![],
    };

    let ast = planner.plan(request).expect("Planning should succeed");

    let mut generator = SQLGenerator::new();
    let generated_query = generator.generate_sql(&ast);

    let expected_query = "SELECT FROM (SELECT username AS username FROM fact_table fact_table LEFT JOIN campaign_hierarchy campaign_hierarchy ON fact_table.line_item_id = campaign_hierarchy.line_item_id WHERE from_unixtime(fact_table.ts, 'YYYY-mm-dd') >= 2020-01-01 AND from_unixtime(fact_table.ts, 'YYYY-mm-dd') < 2021-01-01 GROUP BY from_unixtime(fact_table.ts, 'YYYY-mm-dd'), fact_table.line_item_id, campaign_hierarchy.campaign_id) facts LEFT JOIN dim_campaign dim_campaign ON facts.campaign_id = dim_campaign.campaign_id";
    assert_eq!(generated_query.trim(), expected_query);
}

#[test]
fn test_yaml_conversion() {
    let column: Column = Column {
        name: Rc::from(""),
        column_id: Rc::from(""),
        expression: Rc::from(""),
        column_type: ColumnType::Aggregate,
        data_type: Rc::from(""),
    };

    let yml = serde_yml::to_string(&column).unwrap();

    let ressurected_column: Column = serde_yml::from_str(yml.as_str()).unwrap();

    assert_eq!(ressurected_column, column);
}

#[test]
fn integration_test_query_planner() {
    use reporting::executor::planner::QueryPlanner;
    use reporting::domain::models::{Datasource, Column, ReportRequest, Filter, ColumnType};
    use reporting::rc;

    // Setup a dummy column so that QueryPlanner.get_column can find it.
    let column = Column {
        name: rc!["username"],
        column_id: rc!["username"],
        expression: rc!["username"],
        column_type: ColumnType::Grouping,
        data_type: rc!["text"],
    };

    let datasource = Datasource {
        name: rc!["default"],
        columns: vec![column],
        // add other fields as needed
    };

    let planner = QueryPlanner::new(datasource);
    let request = ReportRequest {
        columns: vec!["username".to_string()],
        filters: Filter::And { value: vec![
            Filter::Gte { column: "date".to_string(), value: "2022-01-01".to_string() },
            Filter::Lt { column: "date".to_string(), value: "2022-12-31".to_string() },
        ]},
        sort: vec![],
        // any additional fields required by ReportRequest
    };

    let plan_result = planner.plan(request);
    assert!(plan_result.is_ok());
}
