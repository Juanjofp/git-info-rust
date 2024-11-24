mod user_mock;
mod repository_mock;
mod event_mock;
mod commit_mock;

use super::{Headers, Methods, Requester, Response};


pub use user_mock::UserJsonMock;
pub use repository_mock::RepositoryJsonMock;
pub use event_mock::EventJsonMock;
pub use commit_mock::CommitJsonMock;


pub mod mocks {

    use std::cell::RefCell;

    use super::{Headers, Methods, Requester, Response};

    pub struct RequesterMock {
        response: RefCell<Vec<Response>>,
    }

    impl RequesterMock {
        pub fn from_response(responses: Vec<Response>) -> Self {
            Self {
                response: RefCell::new(responses),
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
        ) -> Response {
            if self.response.borrow().len() > 0 {
                self.response.borrow_mut().remove(0)
            } else {
                Response::new(0, None)
            }
        }
    }
}
