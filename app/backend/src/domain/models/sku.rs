use crate::domain::models::value_objects::*;
use crate::domain::error::DomainError;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SKU {
    id: SKUId,
    product_id: ProductId,
    sku_code: SKUCode,
    name: SKUName,
    variant_attributes: VariantAttributes,
    base_price: Money,
    sale_price: Option<Money>,
    cost_price: Option<Money>,
    stock: Stock,
    status: SKUStatus,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl SKU {
    pub fn create(
        id: SKUId,
        product_id: ProductId,
        sku_code: SKUCode,
        name: SKUName,
        base_price: Money,
        initial_stock: u32,
    ) -> Result<Self, DomainError> {
        Self::create_with_variants(
            id,
            product_id,
            sku_code,
            name,
            VariantAttributes::new(),
            base_price,
            initial_stock,
        )
    }

    pub fn create_with_variants(
        id: SKUId,
        product_id: ProductId,
        sku_code: SKUCode,
        name: SKUName,
        variant_attributes: VariantAttributes,
        base_price: Money,
        initial_stock: u32,
    ) -> Result<Self, DomainError> {
        let stock = Stock::new(initial_stock, 0)?;

        Ok(Self {
            id,
            product_id,
            sku_code,
            name,
            variant_attributes,
            base_price,
            sale_price: None,
            cost_price: None,
            stock,
            status: SKUStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    // 在庫管理
    pub fn adjust_stock(&mut self, adjustment: StockAdjustment) -> Result<(), DomainError> {
        self.stock.adjust(adjustment)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn reserve_stock(&mut self, quantity: u32) -> Result<(), DomainError> {
        self.stock.reserve(quantity)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn release_reservation(&mut self, quantity: u32) -> Result<(), DomainError> {
        self.stock.release_reservation(quantity)?;
        self.updated_at = Utc::now();
        Ok(())
    }

    // 価格設定
    pub fn set_sale_price(&mut self, sale_price: Money) -> Result<(), DomainError> {
        if sale_price.yen() >= self.base_price.yen() {
            return Err(DomainError::InvalidPrice(
                "Sale price must be less than base price".to_string(),
            ));
        }
        self.sale_price = Some(sale_price);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn clear_sale_price(&mut self) {
        self.sale_price = None;
        self.updated_at = Utc::now();
    }

    pub fn update_base_price(&mut self, price: Money) -> Result<(), DomainError> {
        if !price.is_positive() {
            return Err(DomainError::InvalidPrice(
                "Base price must be positive".to_string(),
            ));
        }

        if let Some(sale_price) = &self.sale_price {
            if sale_price.yen() >= price.yen() {
                return Err(DomainError::InvalidPrice(
                    "Base price must be higher than current sale price".to_string(),
                ));
            }
        }

        self.base_price = price;
        self.updated_at = Utc::now();
        Ok(())
    }

    // ステータス管理
    pub fn activate(&mut self) {
        self.status = SKUStatus::Active;
        self.updated_at = Utc::now();
    }

    pub fn deactivate(&mut self) {
        self.status = SKUStatus::Inactive;
        self.updated_at = Utc::now();
    }

    pub fn discontinue(&mut self) {
        self.status = SKUStatus::Discontinued;
        self.updated_at = Utc::now();
    }

    // Variant関連
    pub fn update_variant_attributes(&mut self, attributes: VariantAttributes) -> Result<(), DomainError> {
        self.variant_attributes = attributes;
        self.updated_at = Utc::now();
        Ok(())
    }

    // ビジネスロジック
    pub fn current_price(&self) -> Money {
        self.sale_price.unwrap_or(self.base_price)
    }

    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some()
    }

    pub fn discount_percentage(&self) -> Option<u8> {
        self.sale_price.map(|sale_price| {
            let discount = self.base_price.yen() - sale_price.yen();
            ((discount as f64 / self.base_price.yen() as f64) * 100.0).round() as u8
        })
    }

    pub fn savings_amount(&self) -> Money {
        if let Some(sale_price) = self.sale_price {
            self.base_price.subtract(sale_price).unwrap_or(Money::from_yen(0))
        } else {
            Money::from_yen(0)
        }
    }

    pub fn is_purchasable(&self) -> bool {
        self.status.is_active() && self.stock.available_quantity() > 0
    }

    pub fn is_low_stock(&self) -> bool {
        self.stock.is_low_stock()
    }

    pub fn is_out_of_stock(&self) -> bool {
        self.stock.available_quantity() == 0
    }

    pub fn full_display_name(&self) -> String {
        if self.variant_attributes.has_any_attributes() {
            format!("{} - {}", self.name.value(), self.variant_attributes.display_name())
        } else {
            self.name.value().to_string()
        }
    }

    pub fn is_simple_sku(&self) -> bool {
        !self.variant_attributes.has_any_attributes()
    }

    // Getters
    pub fn id(&self) -> &SKUId {
        &self.id
    }

    pub fn product_id(&self) -> &ProductId {
        &self.product_id
    }

    pub fn sku_code(&self) -> &SKUCode {
        &self.sku_code
    }

    pub fn name(&self) -> &SKUName {
        &self.name
    }

    pub fn variant_attributes(&self) -> &VariantAttributes {
        &self.variant_attributes
    }

    pub fn base_price(&self) -> Money {
        self.base_price
    }

    pub fn sale_price(&self) -> Option<Money> {
        self.sale_price
    }

    pub fn cost_price(&self) -> Option<Money> {
        self.cost_price
    }

    pub fn stock(&self) -> &Stock {
        &self.stock
    }

    pub fn status(&self) -> &SKUStatus {
        &self.status
    }

    pub fn available_quantity(&self) -> u32 {
        self.stock.available_quantity()
    }

    pub fn total_quantity(&self) -> u32 {
        self.stock.total_quantity()
    }

    pub fn reserved_quantity(&self) -> u32 {
        self.stock.reserved_quantity()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

// Stock構造体
#[derive(Debug, Clone)]
pub struct Stock {
    total_quantity: u32,
    reserved_quantity: u32,
    low_stock_threshold: u32,
}

impl Stock {
    pub fn new(total: u32, reserved: u32) -> Result<Self, DomainError> {
        if reserved > total {
            return Err(DomainError::InvalidStock(
                "Reserved quantity cannot exceed total quantity".to_string(),
            ));
        }

        Ok(Self {
            total_quantity: total,
            reserved_quantity: reserved,
            low_stock_threshold: 5,
        })
    }

    pub fn available_quantity(&self) -> u32 {
        self.total_quantity.saturating_sub(self.reserved_quantity)
    }

    pub fn total_quantity(&self) -> u32 {
        self.total_quantity
    }

    pub fn reserved_quantity(&self) -> u32 {
        self.reserved_quantity
    }

    pub fn adjust(&mut self, adjustment: StockAdjustment) -> Result<(), DomainError> {
        match adjustment {
            StockAdjustment::Increase(amount) => {
                self.total_quantity = self.total_quantity.saturating_add(amount);
            }
            StockAdjustment::Decrease(amount) => {
                if amount > self.available_quantity() {
                    return Err(DomainError::InsufficientStock {
                        requested: amount,
                        available: self.available_quantity(),
                    });
                }
                self.total_quantity -= amount;
            }
        }
        Ok(())
    }

    pub fn reserve(&mut self, quantity: u32) -> Result<(), DomainError> {
        if quantity > self.available_quantity() {
            return Err(DomainError::InsufficientStock {
                requested: quantity,
                available: self.available_quantity(),
            });
        }
        self.reserved_quantity += quantity;
        Ok(())
    }

    pub fn release_reservation(&mut self, quantity: u32) -> Result<(), DomainError> {
        if quantity > self.reserved_quantity {
            return Err(DomainError::InvalidStock(
                "Cannot release more than reserved quantity".to_string(),
            ));
        }
        self.reserved_quantity -= quantity;
        Ok(())
    }

    pub fn set_low_stock_threshold(&mut self, threshold: u32) {
        self.low_stock_threshold = threshold;
    }

    pub fn is_low_stock(&self) -> bool {
        self.available_quantity() > 0 && self.available_quantity() <= self.low_stock_threshold
    }
}

#[derive(Debug, Clone)]
pub enum StockAdjustment {
    Increase(u32),
    Decrease(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SKUStatus {
    Active,
    Inactive,
    Discontinued,
}

impl SKUStatus {
    pub fn is_active(&self) -> bool {
        matches!(self, SKUStatus::Active)
    }
} 