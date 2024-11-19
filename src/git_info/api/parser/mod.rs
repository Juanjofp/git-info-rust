mod user;
mod repository;
mod event;

use super::{constants, ApiError, GitRepositories, GitRepository, GitUser, GitEvents, GitEvent};

pub struct Parser;

impl Parser {

    fn get_body_as_json(body: Option<&String>, url: &str) -> Result<serde_json::Value, ApiError> {
        let url = Some(String::from(url));

        let Some(body) = body else {
            return Err(ApiError::no_body(url));
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
            return Err(ApiError::invalid_json(url));
        };

        Ok(json)
    }
}
