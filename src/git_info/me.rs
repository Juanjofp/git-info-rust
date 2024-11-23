use super::{GitError, GitInfo, GitUser, Requester, Authenticated};

impl<T: Requester> GitInfo<Authenticated, T> {
    pub fn me(&self) -> Result<GitUser, GitError> {
        self.api_service.me().map_err(GitError::from)
    }
}
