use crate::domain::error::DomainError;
use crate::domain::models::value_objects::Money;

/// 商品バリアント（商品の色違い・サイズ違いなど）
#[derive(Debug, Clone, PartialEq)]
pub struct ProductVariant {
    pub id: ProductVariantId,
    pub product_id: ProductVariantProductId,
    pub name: String,
    pub base_price: Money, // セント単位で保存
    pub sale_price: Option<Money>,
    pub color: String,
    pub image_url: Option<String>,
    pub is_available: bool,
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
        name: String,
        base_price: Money,
        sale_price: Option<Money>,
        color: String,
        image_url: Option<String>,
        is_available: bool,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: バリアント名は空不可
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Variant name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 色は空不可
        if color.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Variant color cannot be empty".to_string(),
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

        Ok(Self {
            id,
            product_id,
            name: name.trim().to_string(),
            base_price,
            sale_price,
            color: color.trim().to_string(),
            image_url,
            is_available,
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
        self.is_available
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

    #[test]
    fn create_valid_variant() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let variant = ProductVariant::new(
            id,
            product_id,
            "Small".to_string(),
            Money::from_yen(179000), // 179,000円
            None,
            "Walnut".to_string(),
            Some("https://example.com/image.jpg".to_string()),
            true,
        );
        
        assert!(variant.is_ok());
        let variant = variant.unwrap();
        assert_eq!(variant.current_price().yen(), 179000);
        assert!(!variant.is_on_sale());
        assert!(variant.is_purchasable());
    }

    #[test]
    fn create_variant_with_sale_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let variant = ProductVariant::new(
            id,
            product_id,
            "Small".to_string(),
            Money::from_yen(179000), // 179,000円
            Some(Money::from_yen(149000)), // 149,000円
            "Walnut".to_string(),
            None,
            true,
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
        
        let result = ProductVariant::new(
            id,
            product_id,
            "".to_string(),
            Money::from_yen(179000),
            None,
            "Walnut".to_string(),
            None,
            true,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_empty_color() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "Small".to_string(),
            Money::from_yen(179000),
            None,
            "".to_string(),
            None,
            true,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_zero_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "Small".to_string(),
            Money::from_yen(0), // 0円は無効
            None,
            "Walnut".to_string(),
            None,
            true,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_invalid_sale_price() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        let product_id = ProductVariantProductId::new(1).unwrap();
        
        let result = ProductVariant::new(
            id,
            product_id,
            "Small".to_string(),
            Money::from_yen(100000), // 100,000円
            Some(Money::from_yen(150000)), // 150,000円（基本価格より高い！）
            "Walnut".to_string(),
            None,
            true,
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn reject_zero_product_id() {
        let id = ProductVariantId::new("desk-walnut-small".to_string()).unwrap();
        
        let result = ProductVariantProductId::new(0);
        
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Product ID cannot be zero"));
    }
} 