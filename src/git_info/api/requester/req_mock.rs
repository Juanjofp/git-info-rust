use super::{Headers, Methods, Requester, Response, ResponseError};

#[cfg(test)]
pub mod mocks {

    use super::{Headers, Methods, Requester, Response, ResponseError};

    #[derive(Clone)]
    enum RequesterMockResponse {
        Success(Response),
        Error(ResponseError),
    }

    pub struct RequesterMock {
        response: RequesterMockResponse,
    }

    impl RequesterMock {
        pub fn from_response(response: Response) -> Self {
            RequesterMock {
                response: RequesterMockResponse::Success(response),
            }
        }

        pub fn from_error(error: ResponseError) -> Self {
            RequesterMock {
                response: RequesterMockResponse::Error(error),
            }
        }
    }

    impl Requester for RequesterMock {
        fn fetch(
            &self,
            _method: Methods,
            _url: &str,
            _headers: &Headers,
            _body: Option<String>,
        ) -> Result<super::Response, super::ResponseError> {
            match &self.response {
                RequesterMockResponse::Success(response) => Ok(response.clone()),
                RequesterMockResponse::Error(error) => Err(error.clone()),
            }
        }
    }
}
