use super::{parser::Parser, ApiError, ApiService, Endpoints, GitUserInfo, Methods, Requester};

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn user(&self, username: &str) -> Result<GitUserInfo, ApiError> {
        let url = Endpoints::user(username);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        Parser::user_info(response.body(), &url)
    }
}

#[cfg(test)]
use super::{RequesterMock, RequesterUReq, Response, UserJsonMock};

#[cfg(test)]
mod tests {

    use super::{
        ApiError, ApiService, Parser, RequesterMock, RequesterUReq, Response, UserJsonMock,
    };

    #[test]
    fn test_user_success() {
        let str_response = UserJsonMock::user();

        let expected_user =
            Parser::user_info(Some(&str_response), "https://api.github.com/users/juanjofp")
                .expect("Invalid json");

        let response = Response::new(200, Some(str_response));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, Some(String::from("fake_token")));

        let user = git_info.user("juanjofp").unwrap();

        assert_eq!(user, expected_user);
    }

    #[test]
    fn test_user_not_found() {
        let expected_error =
            ApiError::not_found(Some(String::from("https://api.github.com/users/juanjofp")));

        let response = Response::new(404, None);

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, Some(String::from("fake_token")));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    fn test_user_no_body() {
        let expected_error =
            ApiError::no_body(Some(String::from("https://api.github.com/users/juanjofp")));

        let response = Response::new(200, None);

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, Some(String::from("fake_token")));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    fn test_user_invalid_json() {
        let expected_error =
            ApiError::invalid_json(Some(String::from("https://api.github.com/users/juanjofp")));

        let response = Response::new(200, Some(String::from("invalid json")));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, Some(String::from("fake_token")));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    fn test_user_without_login() {
        let expected_error = ApiError::field_not_found(
            "login",
            Some(String::from("https://api.github.com/users/juanjofp")),
        );

        let str_response = r#"{"id":446496,"node_id":"MDQ6VXNlcjQ0NjQ5Ng==","avatar_url":"https://avatars.githubusercontent.com/u/446496?v=4","gravatar_id":"","url":"https://api.github.com/users/Juanjofp"}"#;

        let response = Response::new(200, Some(String::from(str_response)));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, Some(String::from("fake_token")));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    #[ignore]
    fn test_real_implementation() {
        let requester = RequesterUReq::new();

        let token = Some(String::from("fake_token"));

        let git_info = ApiService::new(requester, token);

        let user = git_info.user("12345432345").unwrap();

        assert_eq!(user.user.login, "juanjofp");
        assert_eq!(user.user.id, "1");
    }
}
