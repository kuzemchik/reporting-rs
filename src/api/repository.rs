use crate::models::models::Datasource;
use crate::models::models::Report;
use sqlx::{Executor, PgPool};
use std::fs;

pub enum PgError {
    Sqlx(sqlx::Error),
    Io(std::io::Error),
}
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Repository {
        Repository { pool }
    }

    pub async fn load_data(&self) -> Result<(i64,), sqlx::Error> {
        sqlx::query_as::<_, (i64,)>("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.pool)
            .await
    }

    pub fn load_datasources(&self) -> Vec<Datasource> {
        let datasources = fs::read_to_string("../test/datasource.yaml").unwrap();
        serde_yml::from_str(datasources.as_str()).unwrap()
    }
    // pub async fn create_report(&self, body: Report) -> Result<Report, PgError> {
    //     sqlx::query("insert into report body values ($1)")
    //         .bind(body)
    //         .execute(&self.pool)
    //         .await
    //         .map_err(PgError::Sqlx)?;
    //     Ok(body)
    // }
}
