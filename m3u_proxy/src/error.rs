use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("HTTP request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("Channel not found: {0}")]
    ChannelNotFound(String),

    #[error("Invalid M3U format: {0}")]
    InvalidM3U(String),

    #[error("Proxy error: {0}")]
    ProxyError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::ChannelNotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidM3U(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::UrlParse(_) => (StatusCode::BAD_REQUEST, self.to_string()),
            AppError::ProxyError(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
