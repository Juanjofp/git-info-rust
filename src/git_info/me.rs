use super::{Authenticated, GitError, GitInfo, GitUserInfo, Requester};

impl<T: Requester> GitInfo<Authenticated, T> {
    pub fn me(&self) -> Result<GitUserInfo, GitError> {
        self.api_service.me().map_err(GitError::from)
    }
}
