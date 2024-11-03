mod api;
mod data;
mod error;

use data::GitUser;

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

    pub fn user(&self, username: &str) -> Result<GitUser, GitError> {
        self.api_service.user(username).map_err(GitError::from)
    }
}
