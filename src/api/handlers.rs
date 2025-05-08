use crate::api::Env;
use crate::domain::models::{Report, ReportStatus};

use axum::extract::{Path, State};
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

#[axum_macros::debug_handler]
pub async fn root(State(_): State<Arc<Env>>) -> impl IntoResponse {
    "Hello, World!"
}

#[axum_macros::debug_handler]
pub async fn get_datasources(State(env): State<Arc<Env>>) -> impl IntoResponse {
    Json(env.repository.load_datasources()).into_response()
}

#[axum_macros::debug_handler]
pub async fn report(
    Path(report_id): Path<String>,
    State(_env): State<Arc<Env>>,
) -> impl IntoResponse {
    // Json(Report::parse(
    //     report_id,
    //     vec!["id", "name"],
    //     ReportStatus::Completed,
    //     None,
    // ))
    // .into_response()
    todo!("Not implemented");
}

#[axum_macros::debug_handler]
pub async fn query(State(_env): State<Arc<Env>>) -> impl IntoResponse {
    todo!("Not implemented");
}
