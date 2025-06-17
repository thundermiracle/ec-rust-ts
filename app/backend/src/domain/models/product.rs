use crate::domain::models::{Category, Color, ProductImage, Tag, ProductVariant};
use crate::domain::error::DomainError;
use crate::domain::models::value_objects::Money;

/// Stock status levels based on business rules
#[derive(Debug, Clone, PartialEq)]
pub enum StockStatus {
    OutOfStock,
    VeryLow,    // 1-5 items
    Low,        // 6-20 items  
    Available,  // 21+ items
}

/// Product display status based on business state
#[derive(Debug, Clone, PartialEq)]
pub enum ProductDisplayStatus {
    SoldOut,
    OnSale,
    Normal,
}

/// Product Aggregate Root (previously ProductAggregate)
/// Clean Architecture: Domain層のAggregate
/// 商品とその関連エンティティを集約し、一貫性を保つ
/// Enhanced Product Model with Japanese Yen pricing and business logic
#[derive(Debug, Clone)]
pub struct Product {
    // Core product fields
    pub id: u32,
    pub name: String,
    pub base_price: Money,         // u32 price → Money base_price
    pub sale_price: Option<Money>, // セール価格（オプション）
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    pub quantity: u32,
    pub is_on_sale: bool,         // セール中
    pub is_available: bool,          // 商品の有効/無効
    pub is_best_seller: bool,     // ベストセラー
    pub is_quick_ship: bool,      // 迅速配送
    
    // Related aggregates
    pub images: Vec<ProductImage>,
    pub category: Category,
    pub colors: Vec<Color>,
    pub tags: Vec<Tag>,
    pub variants: Vec<ProductVariant>,
}

impl Product {
    /// 新しいProductを作成
    pub fn new(
        id: u32,
        name: String,
        description: String,
        quantity: u32,
        base_price: Money,
        category: Category,
        colors: Vec<Color>,
        images: Vec<ProductImage>,
        tags: Vec<Tag>,
        variants: Vec<ProductVariant>,
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
            base_price,
            sale_price: None,
            description: description.trim().to_string(),
            material: None,
            dimensions: None,
            quantity,
            is_on_sale: false,
            is_available: true,
            is_best_seller: false,
            is_quick_ship: false,
            images,
            category,
            colors,
            tags,
            variants,
        })
    }

    /// Simple constructor for cases where related entities are not needed
    pub fn new_simple(
        id: u32, 
        name: String, 
        description: String, 
        quantity: u32,
        base_price: Money
    ) -> Result<Self, DomainError> {
        Self::new(
            id,
            name,
            description,
            quantity,
            base_price,
            Category::default(),
            vec![],
            vec![],
            vec![],
            vec![],
        )
    }

    /// 商品IDを取得
    pub fn product_id(&self) -> u32 {
        self.id
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
        self.is_on_sale = true;
        Ok(())
    }

    /// Clear sale price
    pub fn clear_sale_price(&mut self) {
        self.sale_price = None;
        self.is_on_sale = false;
    }

    /// Set material
    pub fn set_material(&mut self, material: String) {
        self.material = Some(material);
    }

    /// Set dimensions
    pub fn set_dimensions(&mut self, dimensions: String) {
        self.dimensions = Some(dimensions);
    }

    /// Mark as best seller
    pub fn mark_as_best_seller(&mut self) {
        self.is_best_seller = true;
    }

    /// Unmark as best seller
    pub fn unmark_as_best_seller(&mut self) {
        self.is_best_seller = false;
    }

    /// Enable quick ship
    pub fn enable_quick_ship(&mut self) {
        self.is_quick_ship = true;
    }

    /// Disable quick ship
    pub fn disable_quick_ship(&mut self) {
        self.is_quick_ship = false;
    }

    /// Activate product
    pub fn activate(&mut self) {
        self.is_available = true;
    }

    /// Deactivate product
    pub fn deactivate(&mut self) {
        self.is_available = false;
    }

    /// 商品が購入可能かどうかを判定
    pub fn is_available_for_purchase(&self) -> bool {
        self.is_available && self.quantity > 0
    }

    /// Check if product is sold out
    pub fn is_sold_out(&self) -> bool {
        self.quantity == 0
    }

    /// Get stock status based on business rules
    pub fn stock_status(&self) -> StockStatus {
        match self.quantity {
            0 => StockStatus::OutOfStock,
            1..=5 => StockStatus::VeryLow,
            6..=20 => StockStatus::Low,
            _ => StockStatus::Available,
        }
    }

    /// Get product display status based on business state
    pub fn display_status(&self) -> ProductDisplayStatus {
        if self.is_sold_out() {
            ProductDisplayStatus::SoldOut
        } else if self.is_on_sale {
            ProductDisplayStatus::OnSale
        } else {
            ProductDisplayStatus::Normal
        }
    }

    pub fn sell(&mut self, quantity: u32) -> Result<(), DomainError> {
        if !self.is_available {
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

    /// メイン画像を取得
    pub fn main_image(&self) -> Option<&ProductImage> {
        self.images.first()
    }

    /// 色の名前一覧を取得
    pub fn color_names(&self) -> Vec<String> {
        self.colors
            .iter()
            .map(|color| color.name().value().to_string())
            .collect()
    }

    /// タグ名一覧を取得
    pub fn tag_names(&self) -> Vec<String> {
        self.tags
            .iter()
            .map(|tag| tag.name().to_string())
            .collect()
    }

    /// 画像URL一覧を取得
    pub fn image_urls(&self) -> Vec<String> {
        self.images
            .iter()
            .map(|image| image.url().to_string())
            .collect()
    }
} 