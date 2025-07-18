use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::application::commands::models::{
    CreateOrderCommand, CreateOrderCommandCustomerInfo, CreateOrderCommandItem,
    CreateOrderCommandShippingAddress,
};

/// 注文作成用のアイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderRequestItem {
    /// SKU ID
    pub sku_id: String,
    /// 数量
    pub quantity: u32,
}

/// 注文作成用の顧客情報
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderRequestCustomerInfo {
    /// 名
    pub first_name: String,
    /// 姓
    pub last_name: String,
    /// メールアドレス
    pub email: String,
    /// 電話番号
    pub phone: String,
}

/// 注文作成用の配送先情報
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderRequestShippingAddress {
    /// 郵便番号
    pub postal_code: String,
    /// 都道府県
    pub prefecture: String,
    /// 市区町村
    pub city: String,
    /// 住所
    pub street_address: String,
    /// 建物名・部屋番号
    pub building: Option<String>,
}

/// 注文作成リクエスト
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CreateOrderRequest {
    /// 顧客情報
    pub customer_info: CreateOrderRequestCustomerInfo,
    /// 注文アイテム
    pub items: Vec<CreateOrderRequestItem>,
    /// 配送方法ID
    pub shipping_method_id: String,
    /// 支払い方法ID
    pub payment_method_id: String,
    /// 配送先住所
    pub shipping_address: CreateOrderRequestShippingAddress,
}

impl CreateOrderRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.customer_info.first_name.is_empty() {
            return Err("名を入力してください".to_string());
        }
        if self.customer_info.last_name.is_empty() {
            return Err("姓を入力してください".to_string());
        }
        if self.customer_info.email.is_empty() {
            return Err("メールアドレスを入力してください".to_string());
        }
        if self.customer_info.phone.is_empty() {
            return Err("電話番号を入力してください".to_string());
        }
        if self.items.is_empty() {
            return Err("注文アイテムが必要です".to_string());
        }
        for item in &self.items {
            if item.sku_id.is_empty() {
                return Err("SKU IDが必要です".to_string());
            }
            if item.quantity == 0 {
                return Err("数量は1以上である必要があります".to_string());
            }
        }
        if self.shipping_method_id.is_empty() {
            return Err("配送方法IDが必要です".to_string());
        }
        if self.payment_method_id.is_empty() {
            return Err("支払い方法IDが必要です".to_string());
        }
        if self.shipping_address.postal_code.is_empty() {
            return Err("郵便番号を入力してください".to_string());
        }
        if self.shipping_address.prefecture.is_empty() {
            return Err("都道府県を入力してください".to_string());
        }
        if self.shipping_address.city.is_empty() {
            return Err("市区町村を入力してください".to_string());
        }
        if self.shipping_address.street_address.is_empty() {
            return Err("住所を入力してください".to_string());
        }
        Ok(())
    }

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