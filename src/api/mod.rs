use crate::api::repository::Repository;

pub mod handlers;
pub mod repository;

pub struct Env {
    pub repository: Repository,
}
