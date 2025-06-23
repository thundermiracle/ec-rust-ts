use serde::Serialize;

/// API応答用のVariant構造体（VariantDTOに合わせた構造）
#[derive(Debug, Serialize)]
pub struct VariantResponse {
    pub id: String,
    #[serde(rename = "skuCode")]
    pub sku_code: String,
    pub name: String,
    pub color: String,
    pub material: String,
    pub dimensions: String,
    pub price: u32,
    #[serde(rename = "salePrice", skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<u32>,
    // #[serde(rename = "stockQuantity")]
    // pub stock_quantity: u32,
    #[serde(rename = "displayOrder")]
    pub display_order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isOnSale")]
    pub is_on_sale: bool,
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
} 