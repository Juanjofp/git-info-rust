use std::collections::HashMap;

mod req_ureq;
mod response;

pub use response::Response;

pub type Headers = HashMap<String, String>;

pub enum Methods {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

pub trait Requester {
    fn fetch(
        &self,
        method: Methods,
        url: &str,
        headers: &Headers,
        body: Option<String>,
    ) -> Response;
}

#[cfg(test)]
mod req_mock;

#[cfg(test)]
pub use req_mock::mocks::RequesterMock;
