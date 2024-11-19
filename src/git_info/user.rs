use super::{Requester, GitError, GitInfo, GitUser};

impl<T: Requester> GitInfo<T> {
    pub fn user(&self, username: &str) -> Result<GitUser, GitError> {
        self.api_service.user(username).map_err(GitError::from)
    }
}

#[cfg(test)]
use super::{RequesterMock, Response, ApiError};

#[cfg(test)]
mod tests {

    use super::{GitInfo, RequesterMock, Response, GitError, ApiError};

    #[test]
    fn test_user_success() {
      let expected_url = "https://api.github.com/users/juanjofp".to_string();
      let expected_error = ApiError::invalid_json(Some(expected_url));

      let response = Response::new(200, Some(String::from("")));

      let requester = RequesterMock::from_response(vec![response]);

      let git_info = GitInfo::from_requester(requester, "fake_token".to_string());

      let error = git_info.user("juanjofp").unwrap_err();

      assert_eq!(error, GitError::Api(expected_error));
    }
}
