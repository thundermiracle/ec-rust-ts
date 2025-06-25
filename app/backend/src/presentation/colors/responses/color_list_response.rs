use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Color List Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColorListResponse {
    pub colors: Vec<ColorListItemResponse>,
}


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ColorListItemResponse {
    pub id: u32,
    pub name: String,
    pub hex: String,
}
