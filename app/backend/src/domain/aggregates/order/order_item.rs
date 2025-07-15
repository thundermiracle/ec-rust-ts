use crate::domain::value_objects::*;
use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderItem {
    pub sku_id: SKUId,
    pub sku_code: SKUCode,
    pub product_name: ProductName,
    pub sku_name: SKUName,
    pub unit_price: Money,
    pub quantity: i32,
}

impl OrderItem {
    pub fn new(
        sku_id: SKUId,
        sku_code: SKUCode,
        product_name: ProductName,
        sku_name: SKUName,
        unit_price: Money,
        quantity: i32,
    ) -> Result<Self, DomainError> {
        if quantity <= 0 {
            return Err(DomainError::InvalidProductData(
                "Quantity must be positive".to_string()
            ));
        }
        
        if quantity > 999 {
            return Err(DomainError::InvalidProductData(
                "Quantity cannot exceed 999".to_string()
            ));
        }
        
        Ok(OrderItem {
            sku_id,
            sku_code,
            product_name,
            sku_name,
            unit_price,
            quantity,
        })
    }
    
    pub fn subtotal(&self) -> Result<Money, DomainError> {
        self.unit_price.multiply(self.quantity as u32)
    }
    
    pub fn update_quantity(&mut self, new_quantity: i32) -> Result<(), DomainError> {
        if new_quantity <= 0 {
            return Err(DomainError::InvalidProductData(
                "Quantity must be positive".to_string()
            ));
        }
        
        if new_quantity > 999 {
            return Err(DomainError::InvalidProductData(
                "Quantity cannot exceed 999".to_string()
            ));
        }
        
        self.quantity = new_quantity;
        Ok(())
    }
    
    pub fn is_same_sku(&self, sku_id: &SKUId) -> bool {
        self.sku_id == *sku_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_order_item() -> OrderItem {
        OrderItem::new(
            SKUId::new(),
            SKUCode::new("TEST-001".to_string()).unwrap(),
            ProductName::new("Test Product".to_string()).unwrap(),
            SKUName::new("Test SKU".to_string()).unwrap(),
            Money::from_yen(1000),
            2,
        ).unwrap()
    }

    #[test]
    fn test_create_order_item() {
        let item = create_test_order_item();
        assert_eq!(item.quantity, 2);
        assert_eq!(item.unit_price, Money::from_yen(1000));
        assert_eq!(item.subtotal().unwrap(), Money::from_yen(2000));
    }

    #[test]
    fn test_invalid_quantity_zero() {
        let result = OrderItem::new(
            SKUId::new(),
            SKUCode::new("TEST-001".to_string()).unwrap(),
            ProductName::new("Test Product".to_string()).unwrap(),
            SKUName::new("Test SKU".to_string()).unwrap(),
            Money::from_yen(1000),
            0,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_quantity_negative() {
        let result = OrderItem::new(
            SKUId::new(),
            SKUCode::new("TEST-001".to_string()).unwrap(),
            ProductName::new("Test Product".to_string()).unwrap(),
            SKUName::new("Test SKU".to_string()).unwrap(),
            Money::from_yen(1000),
            -1,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_quantity_too_large() {
        let result = OrderItem::new(
            SKUId::new(),
            SKUCode::new("TEST-001".to_string()).unwrap(),
            ProductName::new("Test Product".to_string()).unwrap(),
            SKUName::new("Test SKU".to_string()).unwrap(),
            Money::from_yen(1000),
            1000,
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_update_quantity() {
        let mut item = create_test_order_item();
        
        assert!(item.update_quantity(5).is_ok());
        assert_eq!(item.quantity, 5);
        assert_eq!(item.subtotal().unwrap(), Money::from_yen(5000));
    }

    #[test]
    fn test_update_quantity_invalid() {
        let mut item = create_test_order_item();
        
        assert!(item.update_quantity(0).is_err());
        assert!(item.update_quantity(-1).is_err());
        assert!(item.update_quantity(1000).is_err());
        
        // Original quantity should remain unchanged
        assert_eq!(item.quantity, 2);
    }

    #[test]
    fn test_is_same_sku() {
        let item1 = create_test_order_item();
        let item2_same_sku = OrderItem::new(
            item1.sku_id,
            SKUCode::new("DIFFERENT-CODE".to_string()).unwrap(),
            ProductName::new("Different Product".to_string()).unwrap(),
            SKUName::new("Different SKU".to_string()).unwrap(),
            Money::from_yen(2000),
            1,
        ).unwrap();
        
        let item3_different_sku = create_test_order_item();
        
        assert!(item1.is_same_sku(&item2_same_sku.sku_id));
        assert!(!item1.is_same_sku(&item3_different_sku.sku_id));
    }
}