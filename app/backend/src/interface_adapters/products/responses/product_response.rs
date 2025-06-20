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
    #[serde(rename = "isOnSale", skip_serializing_if = "Option::is_none")]
    pub is_on_sale: Option<bool>,
    #[serde(rename = "isBestSeller", skip_serializing_if = "Option::is_none")]
    pub is_best_seller: Option<bool>,
    #[serde(rename = "isQuickShip", skip_serializing_if = "Option::is_none")]
    pub is_quick_ship: Option<bool>,
    #[serde(rename = "isSoldOut", skip_serializing_if = "Option::is_none")]
    pub is_sold_out: Option<bool>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantResponse>,
} 