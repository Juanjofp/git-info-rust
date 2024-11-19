mod api;
mod data;
mod error;
mod me;
mod repository;
mod user;
mod event;

use data::{GitRepositories, GitUser, GitRepository, GitEvents};

use api::{ApiError, ApiService, RequesterUReq, Requester};

pub use error::GitError;

pub struct GitInfo<T> where T: Requester {
    api_service: ApiService<T>,
}

impl<T> GitInfo<T> where T: Requester {
    pub fn from_requester(requester: T, token: String) -> Self {
        let api_service = ApiService::new(requester, token);

        GitInfo { api_service }
    }
}

impl GitInfo<RequesterUReq> {
  pub fn new(token: String) -> Self {
      let requester = RequesterUReq::new();

      GitInfo::from_requester(requester, token)
  }
}

#[cfg(test)]
use api::{RequesterMock, Response};
