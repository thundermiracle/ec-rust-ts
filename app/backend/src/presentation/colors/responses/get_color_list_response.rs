use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Get Color List Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetColorListResponse {
    pub colors: Vec<GetColorListItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetColorListItemResponse {
    pub id: u32,
    pub name: String,
    pub hex: String,
}
