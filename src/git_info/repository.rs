use super::{GitError, GitInfo, GitRepositories};

impl GitInfo {
    pub fn repositories(&self, username: &str) -> Result<GitRepositories, GitError> {
        self.api_service
            .repositories(username)
            .map_err(GitError::from)
    }
}
