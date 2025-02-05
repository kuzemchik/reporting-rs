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
