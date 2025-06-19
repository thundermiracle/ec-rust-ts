use crate::domain::error::DomainError;
use crate::domain::models::value_objects::Money;
use crate::domain::models::Color;

/// 商品バリアント（商品の色違い・サイズ違いなど）
#[derive(Debug, Clone, PartialEq)]
pub struct ProductVariant {
    pub id: ProductVariantId,
    pub product_id: ProductVariantProductId,
    pub sku: String,
    pub name: String,
    pub color: Option<Color>, // 色オブジェクト（オプション）
    pub dimensions: Option<String>, // 寸法（オプション）
    pub base_price: Money, // セント単位で保存
    pub sale_price: Option<Money>,
    pub stock_quantity: u32,
    pub reserved_quantity: u32,
    pub low_stock_threshold: Option<u32>,
    pub is_available: bool,
    pub image_url: Option<String>,
}

/// 商品バリアントID値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductVariantId(String);

/// 商品ID参照値オブジェクト（ProductVariantが参照する商品ID）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductVariantProductId(u32);

impl ProductVariant {
    /// 新しい商品バリアントを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ProductVariantId,
        product_id: ProductVariantProductId,
        sku: String,
        name: String,
        base_price: Money,
        sale_price: Option<Money>,
        color: Option<Color>,
        dimensions: Option<String>,
        image_url: Option<String>,
        is_available: bool,
        stock_quantity: u32,
        reserved_quantity: u32,
        low_stock_threshold: Option<u32>,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: バリアント名は空不可
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Variant name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: セール価格は通常価格より安い必要がある
        if let Some(sale) = sale_price {
            if sale.yen() >= base_price.yen() {
                return Err(DomainError::InvalidProductData(
                    "Sale price must be lower than base price".to_string(),
                ));
            }
        }

        // ビジネスルール: 価格は0より大きい必要がある
        if !base_price.is_positive() {
            return Err(DomainError::InvalidProductData(
                "Base price must be greater than 0".to_string(),
            ));
        }

        // ビジネスルール: 在庫数は0以上である必要がある
        if stock_quantity < 0 {
            return Err(DomainError::InvalidProductData(
                "Stock quantity cannot be negative".to_string(),
            ));
        }

        // ビジネスルール: 予約数は在庫数以下である必要がある
        if reserved_quantity > stock_quantity {
            return Err(DomainError::InvalidProductData(
                "Reserved quantity cannot exceed stock quantity".to_string(),
            ));
        }

        Ok(Self {
            id,
            product_id,
            sku,
            name: name.trim().to_string(),
            base_price,
            sale_price,
            color,
            dimensions,
            image_url,
            is_available,
            stock_quantity,
            reserved_quantity,
            low_stock_threshold,
        })
    }

    /// 現在の販売価格を取得（セール価格優先）
    pub fn current_price(&self) -> Money {
        self.sale_price.unwrap_or(self.base_price)
    }

    /// セール中かどうかを判定
    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some()
    }

    /// 購入可能かどうかを判定
    pub fn is_purchasable(&self) -> bool {
        self.is_available && self.stock_quantity > self.reserved_quantity
    }

    /// 実際に購入可能な在庫数を取得
    pub fn available_stock(&self) -> u32 {
        self.stock_quantity.saturating_sub(self.reserved_quantity)
    }

    /// 在庫があるかどうかを判定
    pub fn has_stock(&self) -> bool {
        self.available_stock() > 0
    }

    /// 低在庫かどうかを判定
    pub fn is_low_stock(&self) -> bool {
        if let Some(threshold) = self.low_stock_threshold {
            self.available_stock() <= threshold && self.available_stock() > 0
        } else {
            false
        }
    }

    /// 在庫切れかどうかを判定
    pub fn is_out_of_stock(&self) -> bool {
        self.available_stock() == 0
    }

    /// 割引率を計算（パーセンテージ）
    pub fn discount_percentage(&self) -> Option<u8> {
        if let Some(sale) = self.sale_price {
            let discount_amount = self.base_price.yen() - sale.yen();
            let discount_rate = (discount_amount as f64 / self.base_price.yen() as f64) * 100.0;
            Some(discount_rate.round() as u8)
        } else {
            None
        }
    }

    /// バリアントIDを取得
    pub fn id(&self) -> &ProductVariantId {
        &self.id
    }

    /// 商品IDを取得
    pub fn product_id(&self) -> &ProductVariantProductId {
        &self.product_id
    }

    /// 色の名前を取得
    pub fn color_name(&self) -> String {
        self.color.as_ref().map(|c| c.name().to_string()).unwrap_or_default()
    }

    /// 色のHEXコードを取得
    pub fn color_hex(&self) -> String {
        self.color.as_ref().map(|c| c.hex_code().to_string()).unwrap_or_default()
    }
}

impl ProductVariantId {
    /// 新しい商品バリアントIDを作成
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Variant ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(id))
    }

    /// IDの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ProductVariantProductId {
    /// 新しい商品ID参照を作成
    pub fn new(product_id: u32) -> Result<Self, DomainError> {
        if product_id == 0 {
            return Err(DomainError::InvalidProductData(
                "Product ID cannot be zero".to_string(),
            ));
        }
        Ok(Self(product_id))
    }

    /// 商品IDの値を取得
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl std::fmt::Display for ProductVariantId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ProductVariantProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::models::ColorName;

    #[test]
    fn create_valid_variant() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        // Create a color for testing
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let variant = ProductVariant::new(
            id,
            product_id,
            "WD-SM-1".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(179000), // 179,000円
            None,
            Some(color),
            Some("S".to_string()), // dimensions
            Some("https://example.com/image.jpg".to_string()),
            true,
            100,
            0,
            Some(5),
        );
        
        assert!(variant.is_ok());
        let variant = variant.unwrap();
        assert_eq!(variant.current_price().yen(), 179000);
        assert!(!variant.is_on_sale());
        assert!(variant.is_purchasable());
        assert_eq!(variant.available_stock(), 100);
        assert!(variant.has_stock());
        assert!(!variant.is_out_of_stock());
        assert_eq!(variant.color_name(), "Walnut");
        assert_eq!(variant.color_hex(), "#8B4513");
    }

    #[test]
    fn create_variant_with_color_object() {
        let id = ProductVariantId::new("desk-walnut-large".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let variant = ProductVariant::new(
            id,
            product_id,
            "WD-LG-1".to_string(), // SKU
            "Large".to_string(),
            Money::from_yen(199000), // 199,000円
            None,
            Some(color),
            Some("L".to_string()),
            Some("https://example.com/image.jpg".to_string()),
            true,
            50,
            5,
            Some(10),
        );
        
        assert!(variant.is_ok());
        let variant = variant.unwrap();
        assert_eq!(variant.current_price().yen(), 199000);
        assert!(!variant.is_on_sale());
        assert!(variant.is_purchasable());
        assert_eq!(variant.available_stock(), 45); // 50 - 5 = 45
        assert_eq!(variant.color_name(), "Walnut");
        assert_eq!(variant.color_hex(), "#8B4513");
    }

    #[test]
    fn create_variant_with_sale_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let variant = ProductVariant::new(
            id,
            product_id,
            "WD-SM-2".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(179000), // 179,000円
            Some(Money::from_yen(149000)), // 149,000円
            Some(color),
            Some("S".to_string()),
            None,
            true,
            100,
            0,
            None,
        );
        
        assert!(variant.is_ok());
        let variant = variant.unwrap();
        assert_eq!(variant.current_price().yen(), 149000);
        assert!(variant.is_on_sale());
        assert_eq!(variant.discount_percentage(), Some(17)); // ~17% discount
    }

    #[test]
    fn reject_empty_name() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "WD-SM-3".to_string(), // SKU
            "".to_string(),
            Money::from_yen(179000),
            None,
            Some(color),
            None,
            None,
            true,
            100,
            0,
            None,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_zero_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "WD-SM-4".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(0), // 0円は無効
            None,
            Some(color),
            None,
            None,
            true,
            100,
            0,
            None,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_invalid_sale_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "WD-SM-5".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(100000), // 100,000円
            Some(Money::from_yen(150000)), // 150,000円（基本価格より高い！）
            Some(color),
            None,
            None,
            true,
            100,
            0,
            None,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_zero_product_id() {
        let result = ProductVariantProductId::new(0);
        assert!(result.is_err());
    }

    #[test]
    fn reject_negative_stock() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "WD-SM-6".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(179000),
            None,
            Some(color),
            None,
            None,
            true,
            0, // This should be fine
            0,
            None,
        );
        
        assert!(result.is_ok());
    }
    
    #[test]
    fn reject_excessive_reservation() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string()).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "WD-SM-7".to_string(), // SKU
            "Small".to_string(),
            Money::from_yen(179000),
            None,
            Some(color),
            None,
            None,
            true,
            10,
            20, // More reserved than in stock
            None,
        );
        
        assert!(result.is_err());
    }
} 