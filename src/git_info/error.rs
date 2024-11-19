use std::fmt::Display;

use super::ApiError;

#[derive(Debug, PartialEq)]
pub enum GitError {
    Api(ApiError),
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GitError::Api(api_error) => write!(f, "{}", api_error),
        }
    }
}

impl GitError {
    pub fn from(api_error: ApiError) -> Self {
        GitError::Api(api_error)
    }
}
