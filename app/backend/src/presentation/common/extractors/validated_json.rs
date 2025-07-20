use axum::{
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

/// Custom extractor that combines JSON deserialization with validation
pub struct ValidatedJson<T>(pub T);

#[derive(Debug)]
pub enum ValidatedJsonRejection {
    JsonError(JsonRejection),
    ValidationError(validator::ValidationErrors),
}

impl IntoResponse for ValidatedJsonRejection {
    fn into_response(self) -> Response {
        match self {
            ValidatedJsonRejection::JsonError(json_err) => {
                (StatusCode::BAD_REQUEST, format!("JSON parse error: {}", json_err)).into_response()
            }
            ValidatedJsonRejection::ValidationError(validation_err) => {
                let mut errors = Vec::new();
                for (field, field_errors) in validation_err.field_errors() {
                    for error in field_errors {
                        let message = error
                            .message
                            .as_ref()
                            .map(|m| m.to_string())
                            .unwrap_or_else(|| format!("Invalid value for field '{}'", field));
                        errors.push(format!("{}: {}", field, message));
                    }
                }
                (
                    StatusCode::BAD_REQUEST,
                    format!("Validation errors: {}", errors.join(", ")),
                )
                    .into_response()
            }
        }
    }
}

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = ValidatedJsonRejection;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value): Json<T> = Json::from_request(req, state)
            .await
            .map_err(ValidatedJsonRejection::JsonError)?;

        value
            .validate()
            .map_err(ValidatedJsonRejection::ValidationError)?;

        Ok(ValidatedJson(value))
    }
}