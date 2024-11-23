mod api;
mod data;
mod error;
mod me;
mod repository;
mod user;
mod event;

use std::marker::PhantomData;

use data::{GitRepositories, GitUser, GitRepository, GitEvents};

use api::{ApiError, ApiService, RequesterUReq, Requester};

pub use error::GitError;

pub struct Anonymous;
pub struct Authenticated;

pub struct GitInfo<U, T> where T: Requester {
    api_service: ApiService<T>,
    _user: PhantomData<U>,

}

impl<T> GitInfo<Authenticated, T> where T: Requester {
    pub fn from_requester_authenticated(requester: T, token: String) -> Self {
        let api_service = ApiService::new(requester, Some(token));

        GitInfo { api_service, _user: PhantomData }
    }
}

impl GitInfo<Authenticated, RequesterUReq> {
  pub fn authenticated(token: String) -> Self {
      let requester = RequesterUReq::new();

      GitInfo::from_requester_authenticated(requester, token)
  }
}

impl<T> GitInfo<Anonymous, T> where T: Requester {
  pub fn from_requester_anonymous(requester: T) -> Self {
      let api_service = ApiService::new(requester, None);

      GitInfo { api_service, _user: PhantomData }
  }
}

impl GitInfo<Anonymous, RequesterUReq> {
  pub fn anonymous() -> Self {
      let requester = RequesterUReq::new();

      GitInfo::from_requester_anonymous(requester)
  }
}

#[cfg(test)]
use api::{RequesterMock, Response};
