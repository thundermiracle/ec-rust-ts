use crate::application::commands::{CalculateCartCommand, CalculationCartCommandItem};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// HTTP リクエスト用のカートアイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCartItemRequest {
    #[validate(length(min = 1, message = "SKU ID cannot be empty"))]
    pub sku_id: String,
    #[validate(range(min = 1, max = 999, message = "Quantity must be between 1 and 999"))]
    pub quantity: u32,
}

/// HTTP リクエスト用のカート計算
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Validate)]
pub struct CalculateCartRequest {
    #[validate(length(min = 1, message = "Cart items cannot be empty"))]
    #[validate(nested)]
    pub items: Vec<CalculateCartItemRequest>,
    #[validate(length(min = 1, message = "Shipping method ID cannot be empty"))]
    pub shipping_method_id: String,
    #[validate(length(min = 1, message = "Payment method ID cannot be empty"))]
    pub payment_method_id: String,
    pub coupon_code: Option<String>,
}

impl CalculateCartRequest {
    /// アプリケーション層のコマンドに変換
    pub fn to_command(&self) -> CalculateCartCommand {
        let items = self
            .items
            .iter()
            .map(|item| CalculationCartCommandItem {
                sku_id: item.sku_id.clone(),
                quantity: item.quantity,
            })
            .collect();

        CalculateCartCommand::new(
            items,
            self.shipping_method_id.clone(),
            self.payment_method_id.clone(),
            self.coupon_code.clone(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_request_converts_to_command() {
        let request = CalculateCartRequest {
            items: vec![
                CalculateCartItemRequest {
                    sku_id: "sku-123".to_string(),
                    quantity: 2,
                },
                CalculateCartItemRequest {
                    sku_id: "sku-456".to_string(),
                    quantity: 1,
                },
            ],
            shipping_method_id: "standard".to_string(),
            payment_method_id: "credit_card".to_string(),
            coupon_code: None,
        };

        assert!(request.validate().is_ok());

        let command = request.to_command();
        assert_eq!(command.items.len(), 2);
        assert_eq!(command.items[0].sku_id, "sku-123");
        assert_eq!(command.items[0].quantity, 2);
        assert_eq!(command.shipping_method_id, "standard");
        assert_eq!(command.payment_method_id, "credit_card");
    }

    #[test]
    fn empty_items_fails_validation() {
        let request = CalculateCartRequest {
            items: vec![],
            shipping_method_id: "standard".to_string(),
            payment_method_id: "credit_card".to_string(),
            coupon_code: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn zero_quantity_fails_validation() {
        let request = CalculateCartRequest {
            items: vec![CalculateCartItemRequest {
                sku_id: "sku-123".to_string(),
                quantity: 0,
            }],
            shipping_method_id: "standard".to_string(),
            payment_method_id: "credit_card".to_string(),
            coupon_code: None,
        };

        assert!(request.validate().is_err());
    }

    #[test]
    fn duplicate_sku_fails_validation() {
        let request = CalculateCartRequest {
            items: vec![
                CalculateCartItemRequest {
                    sku_id: "sku-123".to_string(),
                    quantity: 1,
                },
                CalculateCartItemRequest {
                    sku_id: "sku-123".to_string(),
                    quantity: 2,
                },
            ],
            shipping_method_id: "standard".to_string(),
            payment_method_id: "credit_card".to_string(),
            coupon_code: None,
        };

        assert!(request.validate().is_err());
    }
}
