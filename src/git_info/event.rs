use super::{GitEvents, GitError, GitInfo, Requester};

impl<U, T: Requester> GitInfo<U, T> {
    pub fn events(&self, username: &str) -> Result<GitEvents, GitError> {
        self.api_service
            .events(username)
            .map_err(GitError::from)
    }
}
