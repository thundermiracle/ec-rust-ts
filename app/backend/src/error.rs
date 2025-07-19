use crate::application::ApplicationError;
use crate::presentation::ErrorResponse;
use axum::{Json, http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BuyProductFailed,
    NotFound,
    InternalServerError,
    ServerError(Option<String>),
    ValidationError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> Error: {:?}", self);

        let (status, error_response) = match self {
            Error::BuyProductFailed => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: "BUY_PRODUCT_FAILED".to_string(),
                    message: "Failed to buy product".to_string(),
                    details: None,
                },
            ),
            Error::NotFound => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: "NOT_FOUND".to_string(),
                    message: "Resource not found".to_string(),
                    details: None,
                },
            ),
            Error::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: "INTERNAL_SERVER_ERROR".to_string(),
                    message: "Internal server error".to_string(),
                    details: None,
                },
            ),
            Error::ServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: "SERVER_ERROR".to_string(),
                    message: msg.unwrap_or_else(|| "Internal server error".to_string()),
                    details: None,
                },
            ),
            Error::ValidationError(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: "VALIDATION_ERROR".to_string(),
                    message: msg,
                    details: None,
                },
            ),
        };

        (status, Json(error_response)).into_response()
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
        println!("->> [ErrorConverter] ApplicationError conversion: {:?}", app_error);

        match app_error {
            ApplicationError::ProductNotFound(_) => Error::NotFound,
            ApplicationError::Domain(domain_error) => {
                println!("->> Domain error details: {:?}", domain_error);
                // ドメインエラーは通常、バリデーションエラーとして扱う
                Error::ValidationError(domain_error.to_string())
            }
            ApplicationError::Repository(repo_error) => {
                println!("->> [ErrorHandler] Repository error details: {:?}", repo_error);
                println!("->> [ErrorHandler] Converting to user message: {}", repo_error.to_user_message());
                match &repo_error {
                    crate::application::error::RepositoryError::NotFound => Error::NotFound,
                    crate::application::error::RepositoryError::ForeignKeyConstraint { .. } => {
                        // 集約されたユーザーメッセージ変換を使用
                        Error::ValidationError(repo_error.to_user_message())
                    }
                    crate::application::error::RepositoryError::QueryExecution(..) => {
                        Error::InternalServerError
                    }
                    crate::application::error::RepositoryError::DatabaseConnection(..) => {
                        Error::InternalServerError
                    }
                    crate::application::error::RepositoryError::DataConversionError(..) => {
                        Error::InternalServerError
                    }
                    crate::application::error::RepositoryError::DatabaseError(..) => {
                        Error::InternalServerError
                    }
                    crate::application::error::RepositoryError::Unknown(..) => {
                        Error::InternalServerError
                    }
                }
            }
            ApplicationError::Validation(msg) => Error::ValidationError(msg),
            ApplicationError::InvalidInput(msg) => Error::ValidationError(msg),
            ApplicationError::NotFound(_) => Error::NotFound,
        }
    }
}
