use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum ApiServiceErrorKind {
    NoResponse,
    NoBody,
    InvalidJson,
    NotFound,
    FieldNotFound(String),
}

impl ApiServiceErrorKind {
    fn message(&self) -> String {
        match self {
            ApiServiceErrorKind::NoResponse => String::from("Server ERROR, no response"),
            ApiServiceErrorKind::NoBody => String::from("No body"),
            ApiServiceErrorKind::InvalidJson => String::from("Error invalid json"),
            ApiServiceErrorKind::NotFound => String::from("Error user not found"),
            ApiServiceErrorKind::FieldNotFound(field) => format!("Error {} not found", field),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ApiError {
    kind: ApiServiceErrorKind,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiError: status: {}, message: {:?}",
            self.kind.message(),
            self.message
        )
    }
}

impl ApiError {
    pub fn new(kind: ApiServiceErrorKind, message: Option<String>) -> Self {
        let message = message.unwrap_or_else(|| kind.message());
        ApiError { kind, message }
    }

    pub fn no_response(message: Option<String>) -> Self {
        ApiError::new(ApiServiceErrorKind::NoResponse, message)
    }

    pub fn no_body(message: Option<String>) -> Self {
        ApiError::new(ApiServiceErrorKind::NoBody, message)
    }

    pub fn invalid_json(message: Option<String>) -> Self {
        ApiError::new(ApiServiceErrorKind::InvalidJson, message)
    }

    pub fn not_found(message: Option<String>) -> Self {
        ApiError::new(ApiServiceErrorKind::NotFound, message)
    }

    pub fn field_not_found(field: &str, message: Option<String>) -> Self {
        ApiError::new(
            ApiServiceErrorKind::FieldNotFound(field.to_string()),
            message,
        )
    }

    pub fn kind(&self) -> &ApiServiceErrorKind {
        &self.kind
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}
