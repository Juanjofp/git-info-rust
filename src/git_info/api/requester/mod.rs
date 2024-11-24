use std::collections::HashMap;

mod req_ureq;
mod response;

pub use response::Response;

pub use req_ureq::RequesterUReq;

pub type Headers = HashMap<String, String>;

pub enum Methods {
    Get,
    // Post,
    // Put,
    // Delete,
    // Patch,
    // Head,
    // Options,
}

impl Methods {
    pub fn as_str(&self) -> &str {
        match self {
            Methods::Get => "GET",
            // Methods::Post => "POST",
            // Methods::Put => "PUT",
            // Methods::Delete => "DELETE",
            // Methods::Patch => "PATCH",
            // Methods::Head => "HEAD",
            // Methods::Options => "OPTIONS",
        }
    }
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
pub use req_mock::{mocks::RequesterMock, UserJsonMock, RepositoryJsonMock, EventJsonMock, CommitJsonMock};
