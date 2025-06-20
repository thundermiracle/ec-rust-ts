use serde::Serialize;

/// API応答用のVariant構造体（mockData.tsのvariantsに合わせた構造）
#[derive(Debug, Serialize)]
pub struct VariantResponse {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    pub image: String,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
} 