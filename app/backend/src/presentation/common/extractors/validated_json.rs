use crate::Error;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

/// Custom extractor that combines JSON deserialization with validation
pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + Send,
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value): Json<T> = Json::from_request(req, state).await.map_err(|json_err| {
            Error::ValidationError(format!("JSON parse error: {}", json_err))
        })?;

        value.validate().map_err(|validation_err| {
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
            Error::ValidationError(format!("入力エラー: {}", errors.join(", ")))
        })?;

        Ok(ValidatedJson(value))
    }
}
