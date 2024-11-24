use super::{ApiError, ApiService, Endpoints, GitEvents, Methods, Parser, Requester};

impl<T> ApiService<T>
where
    T: Requester,
{
    pub fn events(&self, username: &str) -> Result<GitEvents, ApiError> {
        let url = Endpoints::events(username);

        let response = self
            .requester
            .fetch(Methods::Get, &url, &self.headers, None);

        if let Some(error) = self.contains_error(&response, &url) {
            return Err(error);
        }

        Parser::parse_git_events(response.body(), &url)
    }
}

#[cfg(test)]
use super::{EventJsonMock, RequesterMock, RequesterUReq, Response};

#[cfg(test)]
mod tests {

    use super::{ApiService, EventJsonMock, Parser, RequesterMock, RequesterUReq, Response};

    #[test]
    fn test_events_success() {
        let str_response = EventJsonMock::events();

        let expected_events = Parser::parse_git_events(
            Some(&str_response),
            "https://api.github.com/users/juanjofp/events",
        )
        .unwrap();

        let response = Response::new(200, Some(str_response));

        let requester = RequesterMock::from_response(vec![response]);

        let token = String::from("fake_token");

        let git_info = ApiService::new(requester, Some(token));

        let events = git_info.events("juanjofp").unwrap();

        assert_eq!(events.size(), 2);

        let event = events.get(0).unwrap();

        assert_eq!(event, expected_events.get(0).unwrap());
    }

    #[test]
    #[ignore]
    fn test_real_implementation() {
        let requester = RequesterUReq::new();

        let token = String::from("fake_token");

        let git_info = ApiService::new(requester, Some(token));

        let repos = git_info.events("juanjofp").unwrap();

        println!("{:?}", repos);

        assert_eq!(repos.size(), 30);

        repos.get(0).unwrap();
    }
}
