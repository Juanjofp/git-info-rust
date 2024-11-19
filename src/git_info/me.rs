use super::{GitError, GitInfo, GitUser, Requester};

impl<T: Requester> GitInfo<T> {
    pub fn me(&self) -> Result<GitUser, GitError> {
        self.api_service.me().map_err(GitError::from)
    }
}
