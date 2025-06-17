use serde::Serialize;
use super::variant_response::VariantResponse;

/// API応答用のProduct構造体（mockData.tsに合わせた構造）
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,                    // mockDataに合わせて文字列
    pub name: String,
    pub price: u32,                    // base_priceからpriceに変更
    #[serde(rename = "salePrice", skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<u32>,       // salePrice形式
    pub images: Vec<String>,
    pub category: String,              // カテゴリーは文字列（slug）
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    pub colors: Vec<String>,
    #[serde(rename = "isOnSale")]
    pub is_on_sale: bool,
    #[serde(rename = "isBestSeller")]
    pub is_best_seller: bool,
    #[serde(rename = "isQuickShip")]
    pub is_quick_ship: bool,
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantResponse>,
} 