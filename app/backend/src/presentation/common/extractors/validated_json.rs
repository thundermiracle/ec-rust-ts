use crate::Error;
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors, ValidationErrorsKind};

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
            let errors = collect_validation_errors(&validation_err, None);
            Error::ValidationError(errors.join("\n "))
        })?;

        Ok(ValidatedJson(value))
    }
}

/// Recursively collect validation errors from nested structures
fn collect_validation_errors(errors: &ValidationErrors, prefix: Option<&str>) -> Vec<String> {
    let mut error_messages = Vec::new();

    // ValidationErrors.errors() returns an iterator over (field_name, ValidationErrorsKind)
    for (field, error_kind) in errors.errors() {
        let field_name = match prefix {
            Some(p) => format!("{}.{}", p, field),
            None => field.to_string(),
        };

        match error_kind {
            // Field-level validation errors
            ValidationErrorsKind::Field(field_errors) => {
                for error in field_errors {
                    let message = error
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| format!("Invalid value for field '{}'", field_name));
                    // error_messages.push(format!("{}: {}", field_name, message));
                    error_messages.push(message);
                }
            }
            // Nested struct validation errors
            ValidationErrorsKind::Struct(nested_errors) => {
                let nested_error_messages =
                    collect_validation_errors(nested_errors, Some(&field_name));
                error_messages.extend(nested_error_messages);
            }
            // List/array validation errors
            ValidationErrorsKind::List(list_errors) => {
                for (index, nested_errors) in list_errors {
                    let indexed_field_name = format!("{}[{}]", field_name, index);
                    let nested_error_messages =
                        collect_validation_errors(nested_errors, Some(&indexed_field_name));
                    error_messages.extend(nested_error_messages);
                }
            }
        }
    }

    error_messages
}
