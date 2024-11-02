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
