use super::{GitError, GitInfo, GitUser};

impl GitInfo {
    pub fn me(&self) -> Result<GitUser, GitError> {
        self.api_service.me().map_err(GitError::from)
    }
}
