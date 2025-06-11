use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BuyProductFailed,
    NotFound,
    InternalServerError,
    ServerError(Option<String>),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> Error: {:?}", self);

        match self {
            Error::BuyProductFailed => (
                StatusCode::BAD_REQUEST, 
                "Failed to buy product".to_string()
            ),
            Error::NotFound => (
                StatusCode::NOT_FOUND, 
                "Resource not found".to_string()
            ),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                "Internal server error".to_string()
            ),
            Error::ServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR, 
                msg.unwrap_or_else(|| "Internal server error".to_string())
            ),
        }
        .into_response()
    }
}

impl From<anyhow::Error> for Error {
    fn from(error: anyhow::Error) -> Self {
        Error::ServerError(Some(error.to_string()))
    }
}
