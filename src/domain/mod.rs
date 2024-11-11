pub mod models;
pub mod service;

#[cfg(test)]
pub mod tests {
    use crate::domain::models::ReportRequest;
    use crate::domain::tests::TestError::{LoadFile, ParseJson, ParseYaml};
    use serde::de::DeserializeOwned;
    use std::fs;

    #[derive(Debug)]
    pub(crate) enum TestError {
        LoadFile(String, String),
        ParseJson(String, String),
        ParseYaml(String, String),
    }

    pub(crate) fn load_json<T: DeserializeOwned>(
        src: &str,
    ) -> Result<T, TestError> {
        fs::read_to_string(src)
            .map_err(|e| LoadFile(src.to_string(), e.to_string()))
            .and_then(|req| {
                serde_json::from_str(req.as_str())
                    .map_err(|e| ParseJson(req, e.to_string()))
            })
    }

    pub(crate) fn load_yaml<T: DeserializeOwned>(
        src: &str,
    ) -> Result<T, TestError> {
        fs::read_to_string(src)
            .map_err(|e| LoadFile(src.to_string(), e.to_string()))
            .and_then(|ds| {
                serde_yml::from_str(ds.as_str())
                    .map_err(|e| ParseYaml(ds, e.to_string()))
            })
    }
}
