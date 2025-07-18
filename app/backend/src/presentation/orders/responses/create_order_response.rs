use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// 注文作成レスポンス
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderResponse {
    /// 注文ID
    pub order_id: String,
    /// 注文番号
    pub order_number: String,
    /// 合計金額（円）
    pub total_amount: u32,
    /// 注文ステータス
    pub status: String,
}