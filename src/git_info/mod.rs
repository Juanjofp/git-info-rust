mod api;
mod data;
mod error;
mod me;
mod repository;
mod user;

use data::{GitRepositories, GitUser};

use api::{ApiError, ApiService, RequesterUReq};

pub use error::GitError;

pub struct GitInfo {
    api_service: ApiService<RequesterUReq>,
}

impl GitInfo {
    pub fn new(token: String) -> Self {
        let requester = RequesterUReq::new();

        let api_service = ApiService::new(requester, token);

        Self { api_service }
    }
}
