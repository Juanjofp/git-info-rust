use super::{Requester, GitError, GitInfo, GitRepositories, GitRepository};

impl<U, T: Requester> GitInfo<U, T> {
    pub fn repositories(&self, username: &str) -> Result<GitRepositories, GitError> {
        self.api_service
            .repositories(username)
            .map_err(GitError::from)
    }

    pub fn repository(&self, username: &str, repository_name: &str) -> Result<GitRepository, GitError> {
        self.api_service
            .repository(username, repository_name)
            .map_err(GitError::from)
    }
}
