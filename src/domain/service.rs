use crate::domain::models::{
    Column, Datasource, Report, ReportRequest, ReportStatus,
};
use crate::domain::service::Error::ColumnNotFound;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    ColumnNotFound(String),
}

pub struct ReportService {
    datasource: Datasource,
}

impl ReportService {
    pub fn new(datasource: Datasource) -> Self {
        ReportService { datasource }
    }

    pub fn create_report(
        &self,
        report_request: ReportRequest,
    ) -> Result<Report, Error> {
        let id: Rc<str> = Rc::from(Uuid::new_v4().to_string().as_str());
        let columns: Vec<Column> = report_request
            .columns
            .iter()
            .map(|c| self.map_column(c))
            .collect::<Result<Vec<Column>, Error>>()?;

        let status = ReportStatus::Pending;
        let metadata = None;
        let report = Report {
            id,
            columns,
            status,
            metadata,
        };
        Ok(report)
    }

    fn map_column(&self, input: &String) -> Result<Column, Error> {
        self.datasource
            .columns
            .iter()
            .find(|c| c.column_id.to_string() == *input)
            .cloned()
            .ok_or(ColumnNotFound(input.clone()))
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
        let report = report_service
            .create_report(request)
            .expect("Could not create report");

        assert_eq!(report.status, ReportStatus::Pending);
    }
}
