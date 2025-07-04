use crate::domain::value_objects::*;
use crate::domain::error::DomainError;

/// カート内の商品アイテムを表すドメインエンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct CartItem {
    sku_id: SKUId,
    product_id: ProductId,
    product_name: ProductName,
    unit_price: Money,
    quantity: u32,
}

impl CartItem {
    /// 新しいカートアイテムを作成
    pub fn new(
        sku_id: SKUId,
        product_id: ProductId,
        product_name: ProductName,
        unit_price: Money,
        quantity: u32,
    ) -> Result<Self, DomainError> {
        if quantity == 0 {
            return Err(DomainError::InvalidProductData(
                "Cart item quantity must be greater than zero".to_string(),
            ));
        }

        if !unit_price.is_positive() {
            return Err(DomainError::InvalidPrice(
                "Cart item unit price must be positive".to_string(),
            ));
        }

        Ok(Self {
            sku_id,
            product_id,
            product_name,
            unit_price,
            quantity,
        })
    }

    /// 小計を計算（単価 × 数量）
    pub fn subtotal(&self) -> Result<Money, DomainError> {
        self.unit_price.multiply(self.quantity)
    }

    /// 数量を更新
    pub fn update_quantity(&mut self, new_quantity: u32) -> Result<(), DomainError> {
        if new_quantity == 0 {
            return Err(DomainError::InvalidProductData(
                "Cart item quantity must be greater than zero".to_string(),
            ));
        }
        self.quantity = new_quantity;
        Ok(())
    }

    /// 数量を増加
    pub fn increase_quantity(&mut self, additional: u32) -> Result<(), DomainError> {
        let new_quantity = self.quantity.checked_add(additional)
            .ok_or_else(|| DomainError::InvalidProductData("Quantity overflow".to_string()))?;
        self.quantity = new_quantity;
        Ok(())
    }

    /// 数量を減少
    pub fn decrease_quantity(&mut self, reduction: u32) -> Result<(), DomainError> {
        if reduction >= self.quantity {
            return Err(DomainError::InvalidProductData(
                "Cannot reduce quantity below zero".to_string(),
            ));
        }
        self.quantity -= reduction;
        Ok(())
    }

    // Getters
    pub fn sku_id(&self) -> &SKUId {
        &self.sku_id
    }

    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }

    pub fn product_name(&self) -> &ProductName {
        &self.product_name
    }

    pub fn unit_price(&self) -> Money {
        self.unit_price
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn create_test_cart_item() -> CartItem {
        CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            ProductId::from_uuid(Uuid::new_v4()),
            ProductName::new("Test Product".to_string()).unwrap(),
            Money::from_yen(1000),
            2,
        ).unwrap()
    }

    #[test]
    fn create_valid_cart_item() {
        let item = create_test_cart_item();
        assert_eq!(item.quantity(), 2);
        assert_eq!(item.unit_price().yen(), 1000);
    }

    #[test]
    fn reject_zero_quantity() {
        let result = CartItem::new(
            SKUId::new(),
            ProductId::new(),
            ProductName::new("Test".to_string()).unwrap(),
            Money::from_yen(1000),
            0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn calculate_subtotal() {
        let item = create_test_cart_item();
        let subtotal = item.subtotal().unwrap();
        assert_eq!(subtotal.yen(), 2000); // 1000 * 2
    }

    #[test]
    fn update_quantity() {
        let mut item = create_test_cart_item();
        item.update_quantity(5).unwrap();
        assert_eq!(item.quantity(), 5);
    }

    #[test]
    fn increase_quantity() {
        let mut item = create_test_cart_item();
        item.increase_quantity(3).unwrap();
        assert_eq!(item.quantity(), 5); // 2 + 3
    }

    #[test]
    fn decrease_quantity() {
        let mut item = create_test_cart_item();
        item.decrease_quantity(1).unwrap();
        assert_eq!(item.quantity(), 1); // 2 - 1
    }
} 