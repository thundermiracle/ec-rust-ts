use serde::{Deserialize, Serialize};
use utoipa::{ToSchema, schema};
use validator::Validate;
use crate::presentation::common::validators::validate_japanese_postal_code;

use crate::application::commands::models::{
    CreateOrderCommand, CreateOrderCommandCustomerInfo, CreateOrderCommandItem,
    CreateOrderCommandShippingAddress,
};

/// 注文作成用のアイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrderRequestItem {
    /// SKU ID
    #[validate(length(min = 1, message = "SKU ID is required"))]
    #[schema(example = "sku-tshirt-red-m")]
    pub sku_id: String,
    /// 数量
    #[validate(range(min = 1, message = "Quantity must be at least 1"))]
    #[schema(example = 2)]
    pub quantity: u32,
}

/// 注文作成用の顧客情報
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrderRequestCustomerInfo {
    /// 名
    #[validate(length(min = 1, message = "First name is required"))]
    #[schema(example = "太郎")]
    pub first_name: String,
    /// 姓
    #[validate(length(min = 1, message = "Last name is required"))]
    #[schema(example = "山田")]
    pub last_name: String,
    /// メールアドレス
    #[validate(email(message = "Invalid email format"))]
    #[schema(example = "taro.yamada@example.com")]
    pub email: String,
    /// 電話番号
    #[validate(length(min = 1, message = "Phone number is required"))]
    #[schema(example = "090-1234-5678")]
    pub phone: String,
}

/// 注文作成用の配送先情報
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrderRequestShippingAddress {
    /// 郵便番号
    #[validate(custom(function = "validate_japanese_postal_code"))]
    #[schema(example = "123-4567")]
    pub postal_code: String,
    /// 都道府県
    #[validate(length(min = 1, message = "Prefecture is required"))]
    #[schema(example = "東京都")]
    pub prefecture: String,
    /// 市区町村
    #[validate(length(min = 1, message = "City is required"))]
    #[schema(example = "渋谷区")]
    pub city: String,
    /// 住所
    #[validate(length(min = 1, message = "Street address is required"))]
    #[schema(example = "道獠4-1-1")]
    pub street_address: String,
    /// 建物名・部屋番号
    #[schema(example = "山田マンション101号室")]
    pub building: Option<String>,
}

/// 注文作成リクエスト
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateOrderRequest {
    /// 顧客情報
    #[validate(nested)]
    pub customer_info: CreateOrderRequestCustomerInfo,
    /// 注文アイテム
    #[validate(length(min = 1, message = "Order items are required"))]
    #[validate(nested)]
    pub items: Vec<CreateOrderRequestItem>,
    /// 配送方法ID
    #[validate(length(min = 1, message = "Shipping method ID is required"))]
    #[schema(example = "standard")]
    pub shipping_method_id: String,
    /// 支払い方法ID
    #[validate(length(min = 1, message = "Payment method ID is required"))]
    #[schema(example = "credit_card")]
    pub payment_method_id: String,
    /// 配送先住所
    #[validate(nested)]
    pub shipping_address: CreateOrderRequestShippingAddress,
}

impl CreateOrderRequest {

    pub fn to_command(&self) -> CreateOrderCommand {
        CreateOrderCommand::new(
            CreateOrderCommandCustomerInfo {
                first_name: self.customer_info.first_name.clone(),
                last_name: self.customer_info.last_name.clone(),
                email: self.customer_info.email.clone(),
                phone: self.customer_info.phone.clone(),
            },
            self.items
                .iter()
                .map(|item| CreateOrderCommandItem {
                    sku_id: item.sku_id.clone(),
                    quantity: item.quantity,
                })
                .collect(),
            self.shipping_method_id.clone(),
            self.payment_method_id.clone(),
            CreateOrderCommandShippingAddress {
                postal_code: self.shipping_address.postal_code.clone(),
                prefecture: self.shipping_address.prefecture.clone(),
                city: self.shipping_address.city.clone(),
                street_address: self.shipping_address.street_address.clone(),
                building: self.shipping_address.building.clone(),
            },
        )
    }
}