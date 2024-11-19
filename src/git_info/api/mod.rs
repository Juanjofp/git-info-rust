mod constants;
mod endpoints;
mod error;
mod me;
mod parser;
mod repository;
mod requester;
mod user;
mod events;

use std::vec;

use super::data::{GitRepositories, GitRepository, GitUser, GitEvents, GitEvent};

pub use requester::{RequesterUReq, Requester};

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
    pub fn new(requester: T, token: String) -> Self {
        let headers = Headers::from_iter(vec![
            (String::from("Authorization"), format!("Bearer {}", token)),
            (
                String::from("Accept"),
                String::from("application/vnd.github.v3+json"),
            ),
            (String::from("User-Agent"), String::from("jjfp::rust")),
        ]);

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
pub use requester::{RequesterMock, Response, UserJsonMock, RepositoryJsonMock, EventJsonMock};
