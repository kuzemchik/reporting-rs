use crate::domain::models::Column;
use askama::Template;
use serde::Deserialize;

#[repr(u64)]
#[derive(PartialEq, Copy, Clone)]
pub enum Dim {
    LI = 0x1,
    IO = 1 << 1 | Dim::LI as u64,
    CA = 1 << 2 | Dim::IO as u64,
}

impl Dim {
    pub fn isin(&self, to_check: &u64) -> bool {
        match self {
            &dimension => (dimension as u64 & *to_check) == dimension as u64,
            _ => panic!("Invalid dimension"),
        }
    }
}

#[derive(Template, Deserialize)]
#[template(path = "query.sql", escape = "none")]
pub struct ReportQuery {
    columns: Vec<Column>,
    joins: u64,
}

impl ReportQuery {
    pub fn new(columns: Vec<Column>, joins: u64) -> Self {
        Self { columns, joins }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_query_generation() {
        let template =
            ReportQuery::new(vec![], Dim::IO as u64).render().unwrap();
        assert_ne!(template, "");
    }
}
