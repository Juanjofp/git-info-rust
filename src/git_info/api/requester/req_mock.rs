use super::{Headers, Methods, Requester, Response};

#[cfg(test)]
pub mod mocks {

    use super::{Headers, Methods, Requester, Response};

    pub struct RequesterMock {
        response: Response,
    }

    impl RequesterMock {
        pub fn from_response(response: Response) -> Self {
            RequesterMock { response }
        }
    }

    impl Requester for RequesterMock {
        fn fetch(
            &self,
            _method: Methods,
            _url: &str,
            _headers: &Headers,
            _body: Option<String>,
        ) -> Response {
            self.response.clone()
        }
    }
}
