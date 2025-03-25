use crate::domain::models::{
    Column, Datasource, Report, ReportRequest, ReportStatus,
};
use std::rc::Rc;
use uuid::Uuid;

enum Error {}

pub struct ReportService {
    datasource: Datasource,
}

impl ReportService {
    pub fn new(datasource: Datasource) -> Self {
        ReportService { datasource }
    }

    pub fn create_report(&self, request: ReportRequest) -> Report {
        let id: Rc<str> = Uuid::new_v4().to_string().into_boxed_str().into();

        let status = ReportStatus::Pending;
        let metadata = None;
        let report = Report {
            id,
            request,
            status,
            metadata,
        };
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::tests::{load_json, load_yaml};

    #[test]
    fn test_column() {
        // let request_string = .unwrap();
        let request_file = "test/report_request.json";
        let request: ReportRequest =
            load_json(request_file).expect("Could not parse request json");

        let datasource_file = "test/datasource.yaml";
        let datasource: Datasource =
            load_yaml(datasource_file).expect("Could not parse request yaml");

        let report_service = ReportService { datasource };
        let report = report_service.create_report(request);

        assert_eq!(report.status, ReportStatus::Pending);
    }
}
