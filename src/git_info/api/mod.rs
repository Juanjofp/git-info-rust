mod error;
mod requester;
mod user;

use std::vec;

use requester::{Headers, Requester};

pub use requester::RequesterUReq;

use super::data::GitUser;

pub use error::ApiError;

pub struct ApiService<T>
where
    T: Requester,
{
    host: String,
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

        ApiService {
            host: "https://api.github.com".to_string(),
            headers,
            requester,
        }
    }

    fn prepare_url(&self, path: &str) -> String {
        format!("{}{}", self.host, path)
    }

    fn contains_error(&self, response: &requester::Response, url: &str) -> Option<ApiError> {
        let url = Some(String::from(url));

        if response.status() == 0 {
            return Some(ApiError::no_response(url));
        }

        if response.status() == 404 {
            return Some(ApiError::not_found(url));
        }

        None
    }
}

#[cfg(test)]
use requester::RequesterMock;
