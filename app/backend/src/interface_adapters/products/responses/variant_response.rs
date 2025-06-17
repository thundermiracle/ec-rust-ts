use serde::Serialize;
use crate::application::queries::VariantQuery;

/// Product Variant応答構造体（mockData.tsのvariant構造に合わせる）
#[derive(Debug, Serialize)]
pub struct VariantResponse {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}

/// VariantDetailからVariantResponseへの変換実装
impl From<VariantQuery> for VariantResponse {
    fn from(detail: VariantQuery) -> Self {
        VariantResponse {
            id: detail.id,
            name: detail.name,
            price: detail.price,
            color: detail.color,
            image: detail.image,
            is_available: detail.is_available,
        }
    }
} 