use axum::{http::StatusCode, response::IntoResponse};
use crate::application::ApplicationError;

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

// ApplicationErrorからErrorへの変換実装
impl From<ApplicationError> for Error {
    fn from(app_error: ApplicationError) -> Self {
        println!("->> ApplicationError conversion: {:?}", app_error);
        
        match app_error {
            ApplicationError::ProductNotFound(_) => Error::NotFound,
            ApplicationError::Domain(domain_error) => {
                println!("->> Domain error details: {:?}", domain_error);
                // ドメインエラーは通常、バリデーションエラーとして扱う
                Error::BuyProductFailed
            },
            ApplicationError::Repository(repo_error) => {
                println!("->> Repository error details: {:?}", repo_error);
                match repo_error {
                    crate::application::error::RepositoryError::NotFound => Error::NotFound,
                    _ => Error::InternalServerError,
                }
            },
            ApplicationError::Validation(_) => Error::BuyProductFailed,
            ApplicationError::InvalidInput(_) => Error::BuyProductFailed,
            ApplicationError::NotFound(_) => Error::NotFound,
        }
    }
}
