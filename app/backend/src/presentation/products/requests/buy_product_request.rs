use serde::{Deserialize, Serialize};

/// Buy Product Request - 商品購入リクエスト専用DTO
/// Clean Architecture: リクエストの責任を明確化
#[derive(Serialize, Deserialize)]
pub struct BuyProductRequest {
    /// 購入数量
    pub quantity: u32,
}

impl BuyProductRequest {
    /// バリデーション処理（必要に応じて）
    pub fn validate(&self) -> Result<(), String> {
        if self.quantity == 0 {
            return Err("Quantity must be greater than 0".to_string());
        }
        if self.quantity > 1000 {
            return Err("Quantity cannot exceed 1000".to_string());
        }
        Ok(())
    }
} 