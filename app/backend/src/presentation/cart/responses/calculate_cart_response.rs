use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// HTTP レスポンス用のカートアイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCartItemResponse {
    pub sku_id: String,
    pub product_id: String,
    pub product_name: String,
    pub unit_price: u32,
    pub quantity: u32,
    pub subtotal: u32,
}

/// クーポン適用結果レスポンス
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppliedCouponResponse {
    pub coupon_code: String,
    pub coupon_name: String,
    pub discount_amount: u32,
    pub message: String,
}

/// クーポンエラーレスポンス
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]  
pub struct CouponErrorResponse {
    pub coupon_code: Option<String>,
    pub error_message: String,
}

/// HTTP レスポンス用のカート
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCartResponse {
    pub items: Vec<CalculateCartItemResponse>,
    pub total_quantity: u32,
    pub item_count: usize,
    pub subtotal: u32,
    pub tax_amount: u32,
    pub total: u32,
    pub is_empty: bool,
    pub shipping_fee: u32,
    pub payment_fee: u32,
    pub applied_coupon: Option<AppliedCouponResponse>,
    pub coupon_error: Option<CouponErrorResponse>,
}
