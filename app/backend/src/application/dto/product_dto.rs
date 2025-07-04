/// ProductのViewModel（Application層）
/// CQRS Query側専用：Infrastructure層から直接構築
/// パフォーマンス重視でドメインモデルを経由しない
#[derive(Debug, Clone)]
pub struct ProductDTO {
    pub id: String,
    pub name: String,
    pub images: Vec<String>,
    pub category: String,
    pub description: String,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub variants: Vec<VariantDTO>,
}

/// Product variantのビューモデル（Application層）
#[derive(Debug, Clone)]
pub struct VariantDTO {
    pub id: String,
    pub sku_code: String,
    pub name: String,
    pub color: String,
    pub material: String,
    pub dimensions: String,
    pub price: u32,
    pub sale_price: Option<u32>,
    pub stock_quantity: u32,
    pub reserved_quantity: u32,
    pub display_order: u32,
    pub image: Option<String>,
    pub is_on_sale: bool,
    pub is_sold_out: bool,
}

/// Category情報のビューモデル（Application層）
#[derive(Debug, Clone)]
pub struct CategoryViewModel {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl ProductDTO {
    /// CQRS Query側用：Infrastructure層から直接構築
    /// パフォーマンス重視でドメインモデルを経由しない
    pub fn new(
        id: String,
        name: String,
        images: Vec<String>,
        category: String,
        description: String,
        is_best_seller: bool,
        is_quick_ship: bool,
        variants: Vec<VariantDTO>,
    ) -> Self {
        Self {
            id,
            name,
            images,
            category,
            description,
            is_best_seller,
            is_quick_ship,
            variants,
        }
    }



    /// バリアントが存在するかチェック
    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }
}

impl VariantDTO {
    /// Infrastructure層から直接構築用
    pub fn new(
        id: String,
        sku_code: String,
        name: String,
        color: String,
        material: String,
        dimensions: String,
        price: u32,
        sale_price: Option<u32>,
        stock_quantity: u32,
        reserved_quantity: u32,
        display_order: u32,
        image: Option<String>,
    ) -> Self {
        let is_on_sale = sale_price.is_some();
        let is_sold_out = stock_quantity == 0;

        Self {
            id,
            sku_code,
            name,
            color,
            material,
            dimensions,
            price,
            sale_price,
            stock_quantity,
            reserved_quantity,
            display_order,
            image,
            is_on_sale,
            is_sold_out,
        }
    }

    /// 価格計算用のヘルパーメソッド
    pub fn calculate_discount_percentage(&self) -> Option<u32> {
        if let Some(sale_price) = self.sale_price {
            if self.price > sale_price {
                let discount = ((self.price - sale_price) as f64 / self.price as f64) * 100.0;
                Some(discount.round() as u32)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 割引額を計算
    pub fn calculate_savings_amount(&self) -> u32 {
        if let Some(sale_price) = self.sale_price {
            if self.price > sale_price {
                self.price - sale_price
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl CategoryViewModel {
    pub fn new(id: String, name: String, slug: String) -> Self {
        Self { id, name, slug }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_construction_for_cqrs() {
        let view_model = ProductDTO::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            vec!["image1.jpg".to_string()],
            "desks".to_string(),
            "Test description".to_string(),
            true,
            false,
            vec![],
        );

        assert_eq!(view_model.id, "desk-walnut-1");
        assert_eq!(view_model.name, "Desk - Walnut");
        assert!(view_model.is_best_seller);
        assert!(!view_model.is_quick_ship);
    }

    #[test]
    fn test_variant_creation() {
        let variant = VariantDTO::new(
            "variant-1".to_string(),
            "SKU001".to_string(),
            "Small".to_string(),
            "Walnut".to_string(),
            "Wood".to_string(),
            "24x12x30".to_string(),
            1790,
            Some(1500),
            10,
            1,
            0,
            Some("image.jpg".to_string()),
        );

        assert_eq!(variant.id, "variant-1");
        assert_eq!(variant.sku_code, "SKU001");
        assert_eq!(variant.name, "Small");
        assert_eq!(variant.price, 1790);
        assert_eq!(variant.color, "Walnut");
        assert!(variant.is_on_sale);
        assert!(!variant.is_sold_out);
    }

    #[test]
    fn test_variant_discount_calculation() {
        let variant = VariantDTO::new(
            "variant-1".to_string(),
            "SKU001".to_string(),
            "Test Variant".to_string(),
            "Walnut".to_string(),
            "Wood".to_string(),
            "24x12x30".to_string(),
            1000,
            Some(800),
            5,
            1,
            0,
            None,
        );

        assert_eq!(variant.calculate_discount_percentage(), Some(20));
        assert_eq!(variant.calculate_savings_amount(), 200);
    }
}
