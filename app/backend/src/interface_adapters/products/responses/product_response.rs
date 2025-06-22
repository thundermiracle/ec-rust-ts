use serde::Serialize;
use super::variant_response::VariantResponse;

/// API応答用のProduct構造体（ProductViewModelに合わせた構造）
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,
    pub name: String,
    pub images: Vec<String>,
    pub category: String,
    pub description: String,
    #[serde(rename = "isBestSeller")]
    pub is_best_seller: bool,
    #[serde(rename = "isQuickShip")]
    pub is_quick_ship: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantResponse>,
} 