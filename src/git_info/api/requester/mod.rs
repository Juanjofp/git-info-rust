use std::{collections::HashMap, fmt::Display};

#[cfg(test)]
mod req_mock;

#[cfg(test)]
pub use req_mock::mocks::RequesterMock;

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

#[derive(Clone)]
pub struct Response {
    status: u16,
    body: Option<String>,
}

impl Response {
    pub fn new(status: u16, body: Option<String>) -> Self {
        Response { status, body }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn body(&self) -> Option<&String> {
        self.body.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResponseError {
    status: u16,
    message: Option<String>,
}

impl Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ResponseError: status: {}, message: {:?}",
            self.status, self.message
        )
    }
}

impl ResponseError {
    pub fn new(status: u16, message: Option<String>) -> Self {
        ResponseError { status, message }
    }

    pub fn status(&self) -> u16 {
        self.status
    }

    pub fn is_not_found(&self) -> bool {
        self.status == 404
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }
}

pub trait Requester {
    fn fetch(
        &self,
        method: Methods,
        url: &str,
        headers: &Headers,
        body: Option<String>,
    ) -> Result<Response, ResponseError>;
}
