use super::{
    requester::{Methods, Requester},
    ApiError, ApiService, GitUser,
};

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn user(&self, username: &str) -> Result<GitUser, ApiError> {
        let path = format!("/users/{}", username);
        let url = self.prepare_url(&path);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        let Some(body) = response.body() else {
            return Err(ApiError::no_body(Some(url)));
        };

        let Ok(json) = serde_json::from_str::<serde_json::Value>(body) else {
            return Err(ApiError::invalid_json(Some(url)));
        };

        let Some(user) = json["login"].as_str() else {
            return Err(ApiError::field_not_found("login", None));
        };

        // let Some(email) = json["email"].as_str() else {
        //     return Err(ApiError::field_not_found("email", None));
        // };

        let email = json["email"].as_str().unwrap_or("");

        let avatar = json["avatar_url"]
            .as_str()
            .unwrap_or("https://avatars.githubusercontent.com/u/446496?v=4");

        let user = GitUser::new(
            String::from(user),
            String::from(email),
            String::from(avatar),
        );

        Ok(user)
    }
}

#[cfg(test)]
use super::{requester::Response, RequesterMock, RequesterUReq};

#[cfg(test)]
mod tests {

    use super::{ApiError, ApiService, GitUser, RequesterMock, RequesterUReq, Response};

    #[test]
    fn test_user_success() {
        let expected_user = GitUser::new(
            String::from("Juanjofp"),
            String::from("juanjo@juanjofp.com"),
            String::from("https://avatars.githubusercontent.com/u/446496?v=4"),
        );

        let str_response = r#"{"login":"Juanjofp","id":446496,"node_id":"MDQ6VXNlcjQ0NjQ5Ng==","avatar_url":"https://avatars.githubusercontent.com/u/446496?v=4","gravatar_id":"","url":"https://api.github.com/users/Juanjofp","html_url":"https://github.com/Juanjofp","followers_url":"https://api.github.com/users/Juanjofp/followers","following_url":"https://api.github.com/users/Juanjofp/following{/other_user}","gists_url":"https://api.github.com/users/Juanjofp/gists{/gist_id}","starred_url":"https://api.github.com/users/Juanjofp/starred{/owner}{/repo}","subscriptions_url":"https://api.github.com/users/Juanjofp/subscriptions","organizations_url":"https://api.github.com/users/Juanjofp/orgs","repos_url":"https://api.github.com/users/Juanjofp/repos","events_url":"https://api.github.com/users/Juanjofp/events{/privacy}","received_events_url":"https://api.github.com/users/Juanjofp/received_events","type":"User","user_view_type":"public","site_admin":false,"name":"Juanjo","company":"Digio","blog":"http://juanjofp.com","location":"Murcia, Spain","email":"juanjo@juanjofp.com","hireable":null,"bio":null,"twitter_username":null,"notification_email":"juanjo@juanjofp.com","public_repos":62,"public_gists":5,"followers":37,"following":75,"created_at":"2010-10-20T07:04:01Z","updated_at":"2024-10-14T10:54:05Z"}"#;

        let response = Response::new(200, Some(String::from(str_response)));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let user = git_info.user("juanjofp").unwrap();

        assert_eq!(user, expected_user);
    }

    #[test]
    fn test_user_not_found() {
        let expected_error =
            ApiError::not_found(Some(String::from("https://api.github.com/users/juanjofp")));

        let response = Response::new(404, None);

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

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

        let git_info = ApiService::new(requester, String::from("fake_token"));

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

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    fn test_user_without_login() {
        let expected_error = ApiError::field_not_found("login", None);

        let str_response = r#"{"id":446496,"node_id":"MDQ6VXNlcjQ0NjQ5Ng==","avatar_url":"https://avatars.githubusercontent.com/u/446496?v=4","gravatar_id":"","url":"https://api.github.com/users/Juanjofp"}"#;

        let response = Response::new(200, Some(String::from(str_response)));

        let requester = RequesterMock::from_response(vec![response]);

        let git_info = ApiService::new(requester, String::from("fake_token"));

        let response = git_info.user("juanjofp");

        assert!(response.is_err());

        let error = response.unwrap_err();

        assert_eq!(error, expected_error);
    }

    #[test]
    #[ignore]
    fn test_real_implementation() {
        let expected_user = GitUser::new(
            String::from("Juanjofp"),
            String::from("juanjo@juanjofp.com"),
            String::from("https://avatars.githubusercontent.com/u/446496?v=4"),
        );

        let requester = RequesterUReq::new();

        let token = String::from("fake_token");

        let git_info = ApiService::new(requester, token);

        let user = git_info.user("12345432345").unwrap();

        assert_eq!(user, expected_user);
    }
}
