use std::fmt::Display;

use super::ApiError;

#[derive(Debug)]
pub struct GitError {
    message: String,
}

impl Display for GitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GitError: {}", self.message)
    }
}

impl GitError {
    pub fn from(api_error: ApiError) -> Self {
        GitError {
            message: api_error.message().to_string(),
        }
    }
}
