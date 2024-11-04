use crate::models::models::{Column, Datasource, Report, ReportRequest, ReportStatus};
use crate::models::service::Error::ColumnNotFound;
use std::rc::Rc;
use uuid::Uuid;

#[derive(Debug)]
enum Error {
    ColumnNotFound(String),
}

pub struct ReportService {
    datasource: Datasource,
}

impl ReportService {
    pub fn create_report(&self, report_request: ReportRequest) -> Result<Report, Error> {
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
    use crate::models::service::tests::TestError::{LoadFile, ParseJson, ParseYaml};
    use std::cmp::PartialEq;
    use std::fs;
    use std::rc::Rc;

    #[derive(Debug)]
    enum TestError {
        LoadFile(String, String),
        ParseJson(String, String),
        ParseYaml(String, String),
    }

    #[test]
    fn test_column() {
        // let request_string = .unwrap();
        let request_file = "test/report_request.json";
        let request: ReportRequest = fs::read_to_string("test/report_request.json")
            .map_err(|e| LoadFile(request_file.to_string(), e.to_string()))
            .and_then(|req| {
                serde_json::from_str(req.as_str()).map_err(|e| ParseJson(req, e.to_string()))
            })
            .expect("Could not parse request json");

        let datasource_file = "test/datasource.yaml";
        let datasource: Datasource = fs::read_to_string(datasource_file)
            .map_err(|e| LoadFile(datasource_file.to_string(), e.to_string()))
            .and_then(|ds| {
                serde_yml::from_str(ds.as_str()).map_err(|e| ParseYaml(ds, e.to_string()))
            })
            .expect("Could not parse request yaml");

        let report_service = ReportService { datasource };
        let report = report_service
            .create_report(request)
            .expect("Could not create report");

        assert_eq!(report.status, ReportStatus::Pending);
    }
}
