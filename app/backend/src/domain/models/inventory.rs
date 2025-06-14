use crate::domain::error::DomainError;

/// 在庫ドメインモデル
/// 商品とバリアントの在庫管理を担当
#[derive(Debug, Clone, PartialEq)]
pub struct Inventory {
    pub id: InventoryId,
    pub product_id: InventoryProductId,
    pub variant_id: Option<InventoryVariantId>,
    pub quantity: u32,
    pub reserved_quantity: u32,
}

/// 在庫ID値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryId(u32);

/// 商品ID参照値オブジェクト（Inventoryが参照する商品ID）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryProductId(String);

/// バリアントID参照値オブジェクト（Inventoryが参照するバリアントID）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InventoryVariantId(String);

impl Inventory {
    /// 新しい在庫を作成
    pub fn new(
        id: InventoryId,
        product_id: InventoryProductId,
        variant_id: Option<InventoryVariantId>,
        quantity: u32,
        reserved_quantity: u32,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 予約数量は在庫数量を超えてはならない
        if reserved_quantity > quantity {
            return Err(DomainError::InsufficientQuantity {
                requested: reserved_quantity,
                available: quantity,
            });
        }

        Ok(Self {
            id,
            product_id,
            variant_id,
            quantity,
            reserved_quantity,
        })
    }

    /// 在庫なしで新しい在庫エントリを作成
    pub fn empty(
        id: InventoryId,
        product_id: InventoryProductId,
        variant_id: Option<InventoryVariantId>,
    ) -> Self {
        Self {
            id,
            product_id,
            variant_id,
            quantity: 0,
            reserved_quantity: 0,
        }
    }

    /// 販売可能数量を計算
    pub fn available_quantity(&self) -> u32 {
        self.quantity - self.reserved_quantity
    }

    /// 在庫切れかどうかを判定
    pub fn is_sold_out(&self) -> bool {
        self.available_quantity() == 0
    }

    /// 在庫があるかどうかを判定
    pub fn is_in_stock(&self) -> bool {
        !self.is_sold_out()
    }

    /// 指定数量が購入可能かどうかを判定
    pub fn can_purchase(&self, requested_quantity: u32) -> bool {
        self.available_quantity() >= requested_quantity
    }

    /// 在庫を予約
    pub fn reserve(&mut self, quantity: u32) -> Result<(), DomainError> {
        if !self.can_purchase(quantity) {
            return Err(DomainError::InsufficientQuantity {
                requested: quantity,
                available: self.available_quantity(),
            });
        }

        self.reserved_quantity += quantity;
        Ok(())
    }

    /// 予約をキャンセル
    pub fn cancel_reservation(&mut self, quantity: u32) -> Result<(), DomainError> {
        if quantity > self.reserved_quantity {
            return Err(DomainError::InvalidProductData(
                "Cannot cancel more reservations than exist".to_string(),
            ));
        }

        self.reserved_quantity -= quantity;
        Ok(())
    }

    /// 在庫を消費（販売時）
    pub fn consume(&mut self, quantity: u32) -> Result<(), DomainError> {
        if quantity > self.quantity {
            return Err(DomainError::InsufficientQuantity {
                requested: quantity,
                available: self.quantity,
            });
        }

        // 予約数量から優先的に消費
        let from_reserved = std::cmp::min(quantity, self.reserved_quantity);
        let from_available = quantity - from_reserved;

        self.reserved_quantity -= from_reserved;
        self.quantity -= quantity;

        // 追加の予約がある場合は販売可能数量から消費
        if from_available > 0 && self.available_quantity() < from_available {
            return Err(DomainError::InsufficientQuantity {
                requested: from_available,
                available: self.available_quantity(),
            });
        }

        Ok(())
    }

    /// 在庫を補充
    pub fn restock(&mut self, quantity: u32) -> Result<(), DomainError> {
        // オーバーフロー対策
        if self.quantity > u32::MAX - quantity {
            return Err(DomainError::InvalidProductData(
                "Inventory quantity would overflow".to_string(),
            ));
        }

        self.quantity += quantity;
        Ok(())
    }

    /// 在庫状況を文字列で取得
    pub fn status_description(&self) -> String {
        if self.is_sold_out() {
            "Sold Out".to_string()
        } else if self.available_quantity() <= 5 {
            format!("Low Stock ({})", self.available_quantity())
        } else {
            format!("In Stock ({})", self.available_quantity())
        }
    }

    /// バリアント在庫かどうかを判定
    pub fn is_variant_inventory(&self) -> bool {
        self.variant_id.is_some()
    }
}

impl InventoryId {
    /// 新しい在庫IDを作成
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// IDの値を取得
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl InventoryProductId {
    /// 新しい商品ID参照を作成
    pub fn new(product_id: String) -> Result<Self, DomainError> {
        if product_id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Product ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(product_id))
    }

    /// 商品IDの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl InventoryVariantId {
    /// 新しいバリアントID参照を作成
    pub fn new(variant_id: String) -> Result<Self, DomainError> {
        if variant_id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Variant ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(variant_id))
    }

    /// バリアントIDの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for InventoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for InventoryProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for InventoryVariantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_inventory() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        
        let inventory = Inventory::new(id, product_id, None, 10, 2);
        assert!(inventory.is_ok());
        
        let inventory = inventory.unwrap();
        assert_eq!(inventory.available_quantity(), 8);
        assert!(inventory.is_in_stock());
        assert!(!inventory.is_sold_out());
    }

    #[test]
    fn create_variant_inventory() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let variant_id = InventoryVariantId::new("desk-walnut-small".to_string()).unwrap();
        
        let inventory = Inventory::new(id, product_id, Some(variant_id), 5, 0);
        assert!(inventory.is_ok());
        
        let inventory = inventory.unwrap();
        assert!(inventory.is_variant_inventory());
        assert_eq!(inventory.available_quantity(), 5);
    }

    #[test]
    fn reject_invalid_reserved_quantity() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        
        let inventory = Inventory::new(id, product_id, None, 5, 10); // Reserved > Total
        assert!(inventory.is_err());
    }

    #[test]
    fn reserve_inventory() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let mut inventory = Inventory::new(id, product_id, None, 10, 0).unwrap();
        
        assert!(inventory.reserve(3).is_ok());
        assert_eq!(inventory.available_quantity(), 7);
        assert_eq!(inventory.reserved_quantity, 3);
    }

    #[test]
    fn reject_over_reservation() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let mut inventory = Inventory::new(id, product_id, None, 5, 0).unwrap();
        
        assert!(inventory.reserve(10).is_err()); // More than available
    }

    #[test]
    fn consume_inventory() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let mut inventory = Inventory::new(id, product_id, None, 10, 3).unwrap();
        
        assert!(inventory.consume(5).is_ok());
        assert_eq!(inventory.quantity, 5);
        assert_eq!(inventory.reserved_quantity, 0); // Consumed from reserved first
    }

    #[test]
    fn restock_inventory() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let mut inventory = Inventory::new(id, product_id, None, 5, 0).unwrap();
        
        assert!(inventory.restock(10).is_ok());
        assert_eq!(inventory.quantity, 15);
    }

    #[test]
    fn check_purchase_capability() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let inventory = Inventory::new(id, product_id, None, 10, 3).unwrap();
        
        assert!(inventory.can_purchase(5)); // Available: 7
        assert!(inventory.can_purchase(7)); // Exact match
        assert!(!inventory.can_purchase(8)); // More than available
    }

    #[test]
    fn check_sold_out_status() {
        let id = InventoryId::new(1);
        let product_id = InventoryProductId::new("desk-walnut-1".to_string()).unwrap();
        let inventory = Inventory::new(id, product_id, None, 5, 5).unwrap(); // All reserved
        
        assert!(inventory.is_sold_out());
        assert!(!inventory.is_in_stock());
        assert_eq!(inventory.available_quantity(), 0);
    }
} 