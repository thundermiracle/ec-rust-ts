use crate::domain::aggregates::order::Order;
use serde::{Deserialize, Serialize};

/// 注文作成結果DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderResultDTO {
    pub order_id: String,
    pub order_number: String,
    pub total_amount: u32,
    pub status: String,
}

impl CreateOrderResultDTO {
    pub fn from_order(order: &Order) -> Self {
        Self {
            order_id: order.id.to_string(),
            order_number: order.order_number.value().to_string(),
            total_amount: order.pricing.total.yen(),
            status: format!("{:?}", order.status),
        }
    }
}