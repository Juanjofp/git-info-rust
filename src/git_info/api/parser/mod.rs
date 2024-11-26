mod commit;
mod event;
mod repository;
mod user;

use super::{
    constants, ApiError, GitCommit, GitCommits, GitEvent, GitEvents, GitRepositories,
    GitRepository, GitUser,
};

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

    fn get_body_as_json_array(
        body: Option<&String>,
        url: &str,
    ) -> Result<Vec<serde_json::Value>, ApiError> {
        let json = Parser::get_body_as_json(body, url)?;

        let Some(json_array) = json.as_array() else {
            return Err(ApiError::invalid_json(Some(url.to_string())));
        };

        Ok(json_array.to_vec())
    }
}

#[cfg(test)]
use super::CommitJsonMock;
