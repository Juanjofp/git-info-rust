mod requester;
mod user;

use std::vec;

use requester::{Headers, Requester};

use super::data::GitUser;

pub struct GitInfo<T>
where
    T: Requester,
{
    host: String,
    headers: Headers,
    requester: T,
}

impl<T> GitInfo<T>
where
    T: Requester,
{
    pub fn new(requester: T, token: String) -> Self {
        let headers = Headers::from_iter(vec![
            (String::from("Authorization"), format!("Bearer {}", token)),
            (
                String::from("Accept"),
                String::from("application/vnd.github.v3+json"),
            ),
            (String::from("User-Agent"), String::from("jjfp::rust")),
        ]);

        GitInfo {
            host: "https://api.github.com".to_string(),
            headers,
            requester,
        }
    }

    fn prepare_url(&self, path: &str) -> String {
        format!("{}{}", self.host, path)
    }
}

#[cfg(test)]
use requester::RequesterMock;
