use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::application::commands::{CalculateCartCommand, CalculationCartCommandItem};

/// HTTP リクエスト用のカートアイテム
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CalculateCartItemRequest {
    pub sku_id: String,
    pub quantity: u32,
}

/// HTTP リクエスト用のカート計算
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CalculateCartRequest {
    pub items: Vec<CalculateCartItemRequest>,
}

impl CalculateCartRequest {
    /// アプリケーション層のコマンドに変換
    pub fn to_command(&self) -> CalculateCartCommand {
        let items = self.items
            .iter()
            .map(|item| CalculationCartCommandItem {
                sku_id: item.sku_id.clone(),
                quantity: item.quantity,
            })
            .collect();

        CalculateCartCommand::new(items)
    }

    /// バリデーション
    pub fn validate(&self) -> Result<(), String> {
        if self.items.is_empty() {
            return Err("Cart items cannot be empty".to_string());
        }

        for (index, item) in self.items.iter().enumerate() {
            if item.sku_id.trim().is_empty() {
                return Err(format!("Item {} has empty SKU ID", index));
            }

            if item.quantity == 0 {
                return Err(format!("Item {} has zero quantity", index));
            }

            if item.quantity > 999 {
                return Err(format!("Item {} quantity exceeds maximum (999)", index));
            }
        }

        // 重複するSKUのチェック
        let mut seen_skus = std::collections::HashSet::new();
        for item in &self.items {
            if !seen_skus.insert(&item.sku_id) {
                return Err(format!("Duplicate SKU ID: {}", item.sku_id));
            }
        }

        Ok(())
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
        };

        assert!(request.validate().is_ok());
        
        let command = request.to_command();
        assert_eq!(command.items.len(), 2);
        assert_eq!(command.items[0].sku_id, "sku-123");
        assert_eq!(command.items[0].quantity, 2);
    }

    #[test]
    fn empty_items_fails_validation() {
        let request = CalculateCartRequest {
            items: vec![],
        };
        
        assert!(request.validate().is_err());
    }

    #[test]
    fn zero_quantity_fails_validation() {
        let request = CalculateCartRequest {
            items: vec![
                CalculateCartItemRequest {
                    sku_id: "sku-123".to_string(),
                    quantity: 0,
                },
            ],
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
        };
        
        assert!(request.validate().is_err());
    }
} 