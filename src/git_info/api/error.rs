use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum ApiError {
    NoResponse { message: String },
    NoBody { message: String },
    InvalidJson { message: String },
    NotFound { message: String },
    FieldNotFound { message: String, field: String },
}

impl ApiError {
    pub fn no_response(message: Option<String>) -> Self {
        let message = message.unwrap_or("No response from the server".to_string());

        ApiError::NoResponse { message }
    }

    pub fn no_body(message: Option<String>) -> Self {
        let message = message.unwrap_or("No body in the response".to_string());

        ApiError::NoBody { message }
    }

    pub fn invalid_json(message: Option<String>) -> Self {
        let message = message.unwrap_or("Invalid JSON in the response".to_string());

        ApiError::InvalidJson { message }
    }

    pub fn not_found(message: Option<String>) -> Self {
        let message = message.unwrap_or("Resource not found".to_string());

        ApiError::NotFound { message }
    }

    pub fn field_not_found(field: &str, message: Option<String>) -> Self {
        let message = message.unwrap_or(format!("Field {} not found", field));

        ApiError::FieldNotFound {
            message,
            field: field.to_string(),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            ApiError::NoResponse { message } => message,
            ApiError::NoBody { message } => message,
            ApiError::InvalidJson { message } => message,
            ApiError::NotFound { message } => message,
            ApiError::FieldNotFound { message, .. } => message,
        }
    }

    pub fn name(&self) -> &str {
        match &self {
            ApiError::NoResponse { .. } => "NoResponse",
            ApiError::NoBody { .. } => "NoBody",
            ApiError::InvalidJson { .. } => "InvalidJson",
            ApiError::NotFound { .. } => "NotFound",
            ApiError::FieldNotFound { .. } => "FieldNotFound",
        }
    }
}

impl std::error::Error for ApiError {}

impl Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ApiError: status: {}, message: {}",
            self.name(),
            self.message(),
        )
    }
}
