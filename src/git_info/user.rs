use super::{GitError, GitInfo, GitUser};

impl GitInfo {
    pub fn user(&self, username: &str) -> Result<GitUser, GitError> {
        self.api_service.user(username).map_err(GitError::from)
    }
}
