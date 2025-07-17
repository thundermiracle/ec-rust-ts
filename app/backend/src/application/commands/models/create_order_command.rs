use serde::{Deserialize, Serialize};

/// 注文作成用のアイテム
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommandItem {
    pub sku_id: String,
    pub quantity: u32,
}

/// 注文作成用の顧客情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommandCustomerInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
}

/// 注文作成用の配送先情報
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommandShippingAddress {
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street_address: String,
    pub building: Option<String>,
}

/// 注文作成コマンド
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommand {
    pub customer_info: CreateOrderCommandCustomerInfo,
    pub items: Vec<CreateOrderCommandItem>,
    pub shipping_method_id: String,
    pub payment_method_id: String,
    pub shipping_address: CreateOrderCommandShippingAddress,
}

impl CreateOrderCommand {
    pub fn new(
        customer_info: CreateOrderCommandCustomerInfo,
        items: Vec<CreateOrderCommandItem>,
        shipping_method_id: String,
        payment_method_id: String,
        shipping_address: CreateOrderCommandShippingAddress,
    ) -> Self {
        Self {
            customer_info,
            items,
            shipping_method_id,
            payment_method_id,
            shipping_address,
        }
    }
}
