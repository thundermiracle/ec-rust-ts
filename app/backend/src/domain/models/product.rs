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
    pub stock_quantity: u32,
    pub reserved_quantity: u32,
    pub low_stock_threshold: Option<u32>,
    pub is_on_sale: bool,         // セール中
    pub is_available: bool,          // 商品の有効/無効
    pub is_best_seller: bool,     // ベストセラー
    pub is_quick_ship: bool,      // 迅速配送
    
    // Color reference for simple products
    pub color: Option<Color>,
    
    // Flag for variant products
    pub has_variants: bool,
    
    // Related aggregates
    pub images: Vec<ProductImage>,
    pub category: Category,
    pub tags: Vec<Tag>,
    pub variants: Vec<ProductVariant>,
}

impl Product {
    /// 新しいProductを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u32,
        name: String,
        description: String,
        quantity: u32,
        reserved_quantity: u32,
        base_price: Money,
        category: Category,
        color: Option<Color>,
        has_variants: bool,
        images: Vec<ProductImage>,
        tags: Vec<Tag>,
        variants: Vec<ProductVariant>,
        low_stock_threshold: Option<u32>,
    ) -> Result<Self, DomainError> {
        // Validate name
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Product name cannot be empty".to_string(),
            ));
        }

        // Validate price for non-variant products
        if !has_variants {
            if !base_price.is_positive() {
                return Err(DomainError::InvalidProductData(
                    "Product price must be positive".to_string(),
                ));
            }

            // Color validation for non-variant products
            if color.is_none() {
                return Err(DomainError::InvalidProductData(
                    "Simple products must have a color".to_string(),
                ));
            }
        } else {
            // Variant products shouldn't have a direct color
            if color.is_some() {
                return Err(DomainError::InvalidProductData(
                    "Variant products should not have a direct color".to_string(),
                ));
            }

            // Variant products shouldn't have a direct price
            if base_price.yen() > 0 {
                return Err(DomainError::InvalidProductData(
                    "Variant products should not have a direct price".to_string(),
                ));
            }
        }

        // Stock validation
        if reserved_quantity > quantity {
            return Err(DomainError::InvalidProductData(
                "Reserved quantity cannot exceed stock quantity".to_string(),
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
            stock_quantity: quantity,
            reserved_quantity,
            low_stock_threshold,
            is_on_sale: false,
            is_available: true,
            is_best_seller: false,
            is_quick_ship: false,
            color,
            has_variants,
            images,
            category,
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
        reserved_quantity: u32,
        base_price: Money,
        color: Color,
    ) -> Result<Self, DomainError> {
        Self::new(
            id,
            name,
            description,
            quantity,
            reserved_quantity,
            base_price,
            Category::default(),
            Some(color),
            false,  // No variants
            vec![],
            vec![],
            vec![],
            Some(5), // Default low stock threshold
        )
    }

    /// Create a new variant product
    pub fn new_with_variants(
        id: u32, 
        name: String, 
        description: String, 
        variants: Vec<ProductVariant>,
    ) -> Result<Self, DomainError> {
        Self::new(
            id,
            name,
            description,
            0, // No direct quantity for variant products
            0,
            Money::from_yen(0), // No direct price for variant products
            Category::default(),
            None, // No direct color for variant products
            true, // Has variants
            vec![],
            vec![],
            variants,
            None, // No direct threshold for variant products
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
        self.is_available && self.stock_quantity > 0
    }

    /// Check if product is sold out
    pub fn is_sold_out(&self) -> bool {
        self.stock_quantity == 0
    }

    /// Get stock status based on business rules
    pub fn stock_status(&self) -> StockStatus {
        match self.stock_quantity {
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

        if quantity > self.stock_quantity {
            return Err(DomainError::InsufficientQuantity {
                requested: quantity,
                available: self.stock_quantity,
            });
        }
        self.stock_quantity -= quantity;

        Ok(())
    }

    /// Get variant colors for variant products
    pub fn variant_colors(&self) -> Vec<Color> {
        if !self.has_variants {
            return vec![];
        }

        let mut result = Vec::new();
        let mut seen_color_ids = std::collections::HashSet::new();

        // Extract unique colors from variants
        for variant in &self.variants {
            if let Some(color) = &variant.color {
                let color_id = color.id();
                if seen_color_ids.insert(color_id) {
                    result.push(color.clone());
                }
            }
        }

        result
    }

    /// 画像URL一覧を取得
    pub fn image_urls(&self) -> Vec<String> {
        self.images
            .iter()
            .map(|image| image.url().to_string())
            .collect()
    }

    /// 全ての色名一覧を取得
    pub fn all_color_names(&self) -> Vec<String> {
        if self.has_variants {
            self.variant_colors()
                .iter()
                .map(|color| color.name().to_string())
                .collect()
        } else if let Some(color) = &self.color {
            vec![color.name().to_string()]
        } else {
            vec![] // No colors available
        }
    }

    /// Get list of all available color hex codes (either from single color or variants)
    pub fn all_color_hex_codes(&self) -> Vec<String> {
        if self.has_variants {
            return self.variant_colors()
                .iter()
                .map(|color| color.hex_code().to_string())
                .collect();
        } else if let Some(color) = &self.color {
            return vec![color.hex_code().to_string()];
        }
        
        vec![] // No colors available
    }

    /// タグ名一覧を取得
    pub fn tag_names(&self) -> Vec<String> {
        self.tags
            .iter()
            .map(|tag| tag.name().to_string())
            .collect()
    }

    /// メイン画像を取得
    pub fn main_image(&self) -> Option<&ProductImage> {
        self.images.iter().find(|image| image.is_main_image())
    }

    /// 色の名前を取得（単一色の商品の場合）
    pub fn color_name(&self) -> Option<String> {
        self.color.as_ref().map(|c| c.name().to_string())
    }

    /// 色のHEXコードを取得（単一色の商品の場合）
    pub fn color_hex(&self) -> Option<String> {
        self.color.as_ref().map(|c| c.hex_code().to_string())
    }

    /// 実際に購入可能な在庫数を取得
    pub fn available_stock(&self) -> u32 {
        self.stock_quantity.saturating_sub(self.reserved_quantity)
    }
} 