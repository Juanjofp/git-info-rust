use super::{GitCommits, GitError, GitInfo, Requester};

impl<U, T: Requester> GitInfo<U, T> {
    pub fn commits(&self, username: &str, repo: &str) -> Result<GitCommits, GitError> {
        self.api_service
            .commits(username, repo)
            .map_err(GitError::from)
    }
}
