use serde::{Deserialize, Serialize};

/// カート計算用のアイテム
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationCartCommandItem {
    pub sku_id: String,
    pub quantity: u32,
}

/// カート計算コマンド
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateCartCommand {
    pub items: Vec<CalculationCartCommandItem>,
    pub shipping_method_id: String,
    pub payment_method_id: String,
}

impl CalculateCartCommand {
    pub fn new(
        items: Vec<CalculationCartCommandItem>,
        shipping_method_id: String,
        payment_method_id: String,
    ) -> Self {
        Self {
            items,
            shipping_method_id,
            payment_method_id,
        }
    }
}
