use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use reqwest::Error as ReqwestError;
use thiserror::Error;

pub type MyResult<T> = core::result::Result<T, MyError>;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Failed to make a request: {0}")]
    RequestError(#[from] ReqwestError),
    // Add other error variants as needed
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        println!("->> {:<12} - {self:?}", "INTO_RES");

        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
