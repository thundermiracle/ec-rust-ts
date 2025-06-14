use crate::domain::error::DomainError;
use crate::domain::models::value_objects::Money;

/// Enhanced Product Model with Japanese Yen pricing
#[derive(Debug, Clone, PartialEq)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    // 新しく追加するフィールド
    pub base_price: Money,         // u32 price → Money base_price
    pub sale_price: Option<Money>, // セール価格（オプション）
    pub is_active: bool,          // 商品の有効/無効
}

impl Product {
    /// Create a new Product with enhanced pricing
    pub fn new(
        id: u32, 
        name: String, 
        description: String, 
        quantity: u32,
        base_price: Money
    ) -> Result<Self, DomainError> {
        // Validate name
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Product name cannot be empty".to_string(),
            ));
        }

        // Validate price
        if !base_price.is_positive() {
            return Err(DomainError::InvalidProductData(
                "Product price must be positive".to_string(),
            ));
        }

        Ok(Self {
            id,
            name: name.trim().to_string(),
            description: description.trim().to_string(),
            quantity,
            base_price,
            sale_price: None,
            is_active: true,
        })
    }

    /// Create product with legacy u32 price (for backward compatibility)
    pub fn new_legacy(id: u32, name: String, price: u32, description: String, quantity: u32) -> Self {
        Self {
            id,
            name,
            description,
            quantity,
            base_price: Money::from_yen(price),
            sale_price: None,
            is_active: true,
        }
    }

    /// Get the current effective price (sale price if available, otherwise base price)
    pub fn current_price(&self) -> Money {
        self.sale_price.unwrap_or(self.base_price)
    }

    /// Get discount percentage if on sale
    pub fn discount_percentage(&self) -> Option<u8> {
        self.sale_price.map(|sale_price| {
            let discount = self.base_price.yen() - sale_price.yen();
            ((discount as f64 / self.base_price.yen() as f64) * 100.0).ceil() as u8
        })
    }

    /// Calculate total savings amount
    pub fn savings_amount(&self) -> Money {
        if let Some(sale_price) = self.sale_price {
            self.base_price.subtract(sale_price).unwrap_or(Money::from_yen(0))
        } else {
            Money::from_yen(0)
        }
    }

    /// Check if product is on sale
    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some()
    }

    /// Set sale price
    pub fn set_sale_price(&mut self, sale_price: Money) -> Result<(), DomainError> {
        if sale_price.yen() >= self.base_price.yen() {
            return Err(DomainError::InvalidProductData(
                "Sale price must be less than base price".to_string(),
            ));
        }
        self.sale_price = Some(sale_price);
        Ok(())
    }

    /// Clear sale price
    pub fn clear_sale_price(&mut self) {
        self.sale_price = None;
    }

    /// Get legacy price (for backward compatibility)
    pub fn price(&self) -> u32 {
        self.current_price().yen()
    }

    /// Activate product
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    /// Deactivate product
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    /// Check if product is available for purchase
    pub fn is_available_for_purchase(&self) -> bool {
        self.is_active && self.quantity > 0
    }

    pub fn sell(&mut self, quantity: u32) -> Result<(), DomainError> {
        if !self.is_active {
            return Err(DomainError::InvalidProductData(
                "Cannot sell inactive product".to_string(),
            ));
        }

        if quantity > self.quantity {
            return Err(DomainError::InsufficientQuantity {
                requested: quantity,
                available: self.quantity,
            });
        }
        self.quantity -= quantity;

        Ok(())
    }
}