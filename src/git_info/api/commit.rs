use super::{
    endpoints::Endpoints, requester::Methods, ApiError, ApiService, GitCommits, Parser, Requester,
};

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn commits(&self, username: &str, repo: &str) -> Result<GitCommits, ApiError> {
        let url = Endpoints::commits(username, repo);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        Parser::commits(response.body(), &url)
    }
}

#[cfg(test)]
use super::{CommitJsonMock, RequesterMock, Response};

#[cfg(test)]
mod tests {

    use super::{ApiService, CommitJsonMock, RequesterMock, Response};

    #[test]
    pub fn test_commits_success() {
        let response_str = CommitJsonMock::commits();

        let response = Response::new(200, Some(response_str));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, None);

        let commits = git_info.commits("juanjofp", "reponame").unwrap();

        assert_eq!(commits.size(), 1);

        commits.get(0).unwrap();
    }
}
