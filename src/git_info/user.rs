use super::{GitError, GitInfo, GitUserInfo, Requester};

impl<U, T: Requester> GitInfo<U, T> {
    pub fn user(&self, username: &str) -> Result<GitUserInfo, GitError> {
        self.api_service.user(username).map_err(GitError::from)
    }
}

#[cfg(test)]
use super::{ApiError, RequesterMock, Response};

#[cfg(test)]
mod tests {

    use super::{ApiError, GitError, GitInfo, RequesterMock, Response};

    #[test]
    fn test_user_success() {
        let expected_url = "https://api.github.com/users/juanjofp".to_string();
        let expected_error = ApiError::invalid_json(Some(expected_url));

        let response = Response::new(200, Some(String::from("")));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = GitInfo::from_requester_authenticated(requester, String::from("fake_token"));

        let error = git_info.user("juanjofp").unwrap_err();

        assert_eq!(error, GitError::Api(expected_error));
    }
}
