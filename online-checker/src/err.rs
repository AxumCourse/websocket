use axum::{response::IntoResponse, Json};

use crate::resp::JsonResp;

#[derive(Debug)]
pub enum ErrorKind {
    Jwt,
    Config,
    InvalidParameter,
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl Error {
    fn new(kind: ErrorKind, message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
        Self {
            kind,
            message,
            cause,
        }
    }

    pub fn with_cause(kind: ErrorKind, cause: Box<dyn std::error::Error>) -> Self {
        Self::new(kind, cause.to_string(), Some(cause))
    }

    pub fn with_string(kind: ErrorKind, message: String) -> Self {
        Self::new(kind, message, None)
    }
    pub fn with_str(kind: ErrorKind, msg: &str) -> Self {
        Self::with_string(kind, msg.to_string())
    }

    pub fn code(&self) -> i32 {
        -1
    }

    pub fn invalid_parameter(msg: &str) -> Self {
        Self::with_str(ErrorKind::InvalidParameter, msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self::with_cause(ErrorKind::Jwt, Box::new(e))
    }
}

impl From<::config::ConfigError> for Error {
    fn from(e: ::config::ConfigError) -> Self {
        Self::with_cause(ErrorKind::Config, Box::new(e))
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        let resp = JsonResp::err(self);
        Json(resp).into_response()
    }
}
