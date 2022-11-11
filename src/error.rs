use std::env;

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("failed to load an environmental variable: {0}")]
    EnvVar(#[from] env::VarError),
    #[error("invalid address: {0}")]
    InvalidAddr(#[from] std::net::AddrParseError),
    #[error("could not bind to the address")]
    Bind,
    #[error("could not connect to the backend. is it running?: {0}")]
    SocketConnect(std::io::Error),
    #[error("socket error: {0}")]
    Socket(#[from] std::io::Error),
    #[error("invalid parameters")]
    Params,
    #[error("invalid response from backend: {0}")]
    BackendInvalidResponse(serde_json::Error),
    #[error("database error: {0}")]
    Database(anyhow::Error),
    #[error("other error")]
    OtherInternal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        /*
        let pair = match self {
            AppError::OtherInternal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::Socket(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::BackendInvalidResponse(${0:_}) => todo!(),
        };
        pair.into_response()
        */
        (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()).into_response()
    }
}
