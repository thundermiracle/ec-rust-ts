use serde::Serialize;
use utoipa::ToSchema;

/// エラーレスポンス用DTO
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    /// エラーコード
    pub code: String,
    /// エラーメッセージ
    pub message: String,
    /// エラー詳細（任意）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(
        code: impl Into<String>,
        message: impl Into<String>,
        details: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            message: message.into(),
            details: Some(details.into()),
        }
    }
} 