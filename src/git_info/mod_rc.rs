mod requester;
mod user;

use std::rc::Rc;

trait Requester {}

pub struct GitInfo {
    token: String,
    host: String,
    requester: Rc<dyn Requester>,
}

impl GitInfo {
    pub fn new(requester: Rc<dyn Requester>, token: String) -> Self {
        GitInfo {
            token,
            host: "https://api.github.com".to_string(),
            requester,
        }
    }

    fn authorization(&self) -> String {
        format!("Bearer {}", self.token)
    }

    fn add_headers(&self, req: ureq::Request) -> ureq::Request {
        req.set("Authorization", &self.authorization())
            .set("Accept", "application/vnd.github.v3+json")
            .set("User-Agent", "jjfp::rust")
    }

    fn prepare_url(&self, path: &str) -> String {
        format!("{}{}", self.host, path)
    }
}
