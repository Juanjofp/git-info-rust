use super::{ApiService, Requester};

impl<T> ApiService<T> where T: Requester {}