use super::{Headers, Methods, Requester, Response};

pub struct RequesterUReq {}

impl RequesterUReq {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RequesterUReq {
    fn default() -> Self {
        Self::new()
    }
}

impl Requester for RequesterUReq {
    fn fetch(
        &self,
        method: Methods,
        url: &str,
        headers: &Headers,
        body: Option<String>,
    ) -> Response {
        let request = ureq::request(method.as_str(), url);

        let request = headers
            .iter()
            .fold(request, |request, (key, value)| request.set(key, value));

        // println!("Request: {:?}", request);

        let response = if let Some(body) = body {
            request.send_string(&body)
        } else {
            request.call()
        };

        match response {
            Ok(response) => {
                let status = response.status();
                let body = response.into_string();

                // println!("Response {}: {:?}", status, body);

                Response::new(status, body.ok())
            }
            Err(error) => {
                println!("Error ureq: {}", error);

                match error {
                    ureq::Error::Status(status, response) => {
                        let body = response.into_string().unwrap_or_default();

                        Response::new(status, Some(body))
                    }
                    _ => Response::new(0, None),
                }
            }
        }
    }
}
