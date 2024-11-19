use crate::git_info::data::GitRepository;

use super::{ApiError, ApiService, Endpoints, GitRepositories, Methods, Parser, Requester};

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn repositories(&self, username: &str) -> Result<GitRepositories, ApiError> {
        let url = Endpoints::repositories(username);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        Parser::parse_git_repositories(response.body(), &url)
    }

    pub fn repository(&self, username: &str, repository: &str) -> Result<GitRepository, ApiError> {
        let url = Endpoints::repository(username, repository);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        Parser::parse_git_repository(response.body(), &url)
    }
}

#[cfg(test)]
use super::{RequesterMock, RequesterUReq, Response, RepositoryJsonMock};

#[cfg(test)]
mod tests {

    use super::{ApiError, ApiService, Parser, RequesterMock, RequesterUReq, Response, RepositoryJsonMock};

    #[test]
    fn test_repositories_success() {
        let str_response = RepositoryJsonMock::repositories();

        let expected_repositories = Parser::parse_git_repositories(
            Some(&str_response),
            "https://api.github.com/users/juanjofp/repos",
        )
        .unwrap();

        let response = Response::new(200, Some(str_response));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let repos = git_info.repositories("juanjofp").unwrap();

        assert_eq!(repos.size(), 1);

        let repo = repos.get_repository(0).unwrap();

        assert_eq!(repo, expected_repositories.get_repository(0).unwrap());
    }

    #[test]
    fn test_empty_repositories_success() {
        let str_response = r#"[]"#;

        let response = Response::new(200, Some(String::from(str_response)));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let repos = git_info.repositories("juanjofp").unwrap();

        assert_eq!(repos.size(), 0);

        assert!(repos.get_repository(0).is_none());
    }

    #[test]
    fn test_repositories_not_found() {
        let expected_error = ApiError::not_found(Some(String::from(
            "https://api.github.com/users/juanjofp/repos",
        )));

        let response = Response::new(404, None);

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let response = git_info.repositories("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    #[ignore]
    fn test_real_implementation() {
        let requester = RequesterUReq::new();

        let token = String::from("fake_token");

        let git_info = ApiService::new(requester, token);

        let repos = git_info.repositories("juanjofp").unwrap();

        println!("{:?}", repos);

        assert_eq!(repos.size(), 30);

        repos.get_repository(0).unwrap();
    }

    #[test]
    fn test_repository_success() {
        let str_response = RepositoryJsonMock::repository();

        let expected_repository = Parser::parse_git_repository(
            Some(&str_response),
            "https://api.github.com/users/juanjofp/repos",
        )
        .unwrap();

        let response = Response::new(200, Some(str_response));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let repo = git_info.repository("juanjofp", "Hello-World").unwrap();

        assert_eq!(repo, expected_repository);
    }
}
