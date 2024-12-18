mod commit;
mod constants;
mod endpoints;
mod error;
mod events;
mod me;
mod parser;
mod repository;
mod requester;
mod user;

use std::vec;

use super::{
    GitCommit, GitCommits, GitEvent, GitEvents, GitRepositories, GitRepository, GitUser,
    GitUserInfo, GitUserRepos,
};

pub use requester::{Requester, RequesterUReq};

pub use error::ApiError;

use requester::{Headers, Methods};

use endpoints::Endpoints;

use parser::Parser;

pub struct ApiService<T>
where
    T: Requester,
{
    headers: Headers,
    requester: T,
}

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn new(requester: T, token: Option<String>) -> Self {
        let mut headers_vector = vec![];

        headers_vector.push((
            "Accept".to_string(),
            "application/vnd.github.v3+json".to_string(),
        ));
        headers_vector.push(("User-Agent".to_string(), "jjfp::rust".to_string()));

        if let Some(token) = token {
            headers_vector.push(("Authorization".to_string(), format!("Bearer {}", token)));
        }

        let headers = Headers::from_iter(headers_vector);

        ApiService { headers, requester }
    }

    fn contains_error(&self, response: &requester::Response, url: &str) -> Option<ApiError> {
        let url = Some(String::from(url));

        if response.status() == 0 {
            return Some(ApiError::no_response(url));
        }

        if response.status() == 404 {
            return Some(ApiError::not_found(url));
        }

        if response.status() == 401 {
            return Some(ApiError::no_response(url));
        }

        None
    }
}

#[cfg(test)]
pub use requester::{
    CommitJsonMock, EventJsonMock, RepositoryJsonMock, RequesterMock, Response, UserJsonMock,
};
