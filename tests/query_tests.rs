use crate::common::{load_json, load_yaml};
use reporting::domain::models::{Datasource, ReportRequest, ReportStatus};
use reporting::domain::service::ReportService;

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
    let report = report_service
        .create_report(request)
        .expect("Could not create report");

    assert_eq!(report.status, ReportStatus::Pending);
}
