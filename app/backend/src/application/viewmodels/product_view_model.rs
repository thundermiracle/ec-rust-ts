use crate::domain::models::{Product, ProductId, CategoryId};

/// ProductのViewModel（Application層）
/// CQRS Query側専用：Infrastructure層から直接構築
/// パフォーマンス重視でドメインモデルを経由しない
#[derive(Debug, Clone)]
pub struct ProductViewModel {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub sale_price: Option<u32>,
    pub images: Vec<String>,
    pub category: String,
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    pub colors: Vec<String>,
    pub is_on_sale: bool,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub is_sold_out: bool,
    pub variants: Vec<VariantViewModel>,
}

/// Product variantのビューモデル（Application層）
#[derive(Debug, Clone)]
pub struct VariantViewModel {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    pub image: Option<String>,
    pub is_available: bool,
}

/// Category情報のビューモデル（Application層）
#[derive(Debug, Clone)]
pub struct CategoryViewModel {
    pub id: String,
    pub name: String,
    pub slug: String,
}

impl ProductViewModel {
    /// CQRS Query側用：Infrastructure層から直接構築
    /// パフォーマンス重視でドメインモデルを経由しない
    pub fn new(
        id: String,
        name: String,
        price: u32,
        sale_price: Option<u32>,
        images: Vec<String>,
        category: String,
        description: String,
        material: Option<String>,
        dimensions: Option<String>,
        colors: Vec<String>,
        is_on_sale: bool,
        is_best_seller: bool,
        is_quick_ship: bool,
        is_sold_out: bool,
        variants: Vec<VariantViewModel>,
    ) -> Self {
        Self {
            id,
            name,
            price,
            sale_price,
            images,
            category,
            description,
            material,
            dimensions,
            colors,
            is_on_sale,
            is_best_seller,
            is_quick_ship,
            is_sold_out,
            variants,
        }
    }

    /// Builder pattern for Infrastructure layer query construction
    pub fn builder() -> ProductViewModelBuilder {
        ProductViewModelBuilder::default()
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

    /// バリアントが存在するかチェック
    pub fn has_variants(&self) -> bool {
        !self.variants.is_empty()
    }

    /// 購入可能かチェック
    pub fn is_available_for_purchase(&self) -> bool {
        !self.is_sold_out && (self.variants.is_empty() || self.variants.iter().any(|v| v.is_available))
    }
}

impl VariantViewModel {
    /// Infrastructure層から直接構築用
    pub fn new(
        id: String,
        name: String,
        price: u32,
        color: String,
        image: Option<String>,
        is_available: bool,
    ) -> Self {
        Self {
            id,
            name,
            price,
            color,
            image,
            is_available,
        }
    }
}

impl CategoryViewModel {
    pub fn new(id: String, name: String, slug: String) -> Self {
        Self { id, name, slug }
    }
}

/// ProductViewModel構築用のBuilder
/// Infrastructure層での複雑なクエリ結果からViewModelを構築する際に使用
#[derive(Default)]
pub struct ProductViewModelBuilder {
    id: Option<String>,
    name: Option<String>,
    price: Option<u32>,
    sale_price: Option<u32>,
    images: Vec<String>,
    category: Option<String>,
    description: Option<String>,
    material: Option<String>,
    dimensions: Option<String>,
    colors: Vec<String>,
    is_on_sale: bool,
    is_best_seller: bool,
    is_quick_ship: bool,
    is_sold_out: bool,
    variants: Vec<VariantViewModel>,
}

impl ProductViewModelBuilder {
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn price(mut self, price: u32) -> Self {
        self.price = Some(price);
        self
    }

    pub fn sale_price(mut self, sale_price: Option<u32>) -> Self {
        self.sale_price = sale_price;
        self
    }

    pub fn images(mut self, images: Vec<String>) -> Self {
        self.images = images;
        self
    }

    pub fn category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn material(mut self, material: Option<String>) -> Self {
        self.material = material;
        self
    }

    pub fn dimensions(mut self, dimensions: Option<String>) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn colors(mut self, colors: Vec<String>) -> Self {
        self.colors = colors;
        self
    }

    pub fn is_on_sale(mut self, is_on_sale: bool) -> Self {
        self.is_on_sale = is_on_sale;
        self
    }

    pub fn is_best_seller(mut self, is_best_seller: bool) -> Self {
        self.is_best_seller = is_best_seller;
        self
    }

    pub fn is_quick_ship(mut self, is_quick_ship: bool) -> Self {
        self.is_quick_ship = is_quick_ship;
        self
    }

    pub fn is_sold_out(mut self, is_sold_out: bool) -> Self {
        self.is_sold_out = is_sold_out;
        self
    }

    pub fn variants(mut self, variants: Vec<VariantViewModel>) -> Self {
        self.variants = variants;
        self
    }

    pub fn build(self) -> ProductViewModel {
        ProductViewModel {
            id: self.id.unwrap_or_default(),
            name: self.name.unwrap_or_default(),
            price: self.price.unwrap_or_default(),
            sale_price: self.sale_price,
            images: self.images,
            category: self.category.unwrap_or_default(),
            description: self.description.unwrap_or_default(),
            material: self.material,
            dimensions: self.dimensions,
            colors: self.colors,
            is_on_sale: self.is_on_sale,
            is_best_seller: self.is_best_seller,
            is_quick_ship: self.is_quick_ship,
            is_sold_out: self.is_sold_out,
            variants: self.variants,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_construction_for_cqrs() {
        let view_model = ProductViewModel::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            2290,
            Some(1790),
            vec!["image1.jpg".to_string()],
            "desks".to_string(),
            "Test description".to_string(),
            Some("Walnut Wood".to_string()),
            Some("48\" x 24\" x 30\"".to_string()),
            vec!["Walnut".to_string()],
            true,
            true,
            false,
            false,
            vec![],
        );

        assert_eq!(view_model.id, "desk-walnut-1");
        assert_eq!(view_model.name, "Desk - Walnut");
        assert_eq!(view_model.price, 2290);
        assert_eq!(view_model.sale_price, Some(1790));
        assert!(view_model.is_on_sale);
        assert!(view_model.is_best_seller);
    }

    #[test]
    fn test_builder_pattern() {
        let view_model = ProductViewModel::builder()
            .id("desk-walnut-1".to_string())
            .name("Desk - Walnut".to_string())
            .price(2290)
            .sale_price(Some(1790))
            .category("desks".to_string())
            .description("Test description".to_string())
            .is_on_sale(true)
            .is_best_seller(true)
            .build();

        assert_eq!(view_model.id, "desk-walnut-1");
        assert_eq!(view_model.name, "Desk - Walnut");
        assert_eq!(view_model.price, 2290);
        assert!(view_model.is_on_sale);
        assert!(view_model.is_best_seller);
    }

    #[test]
    fn test_discount_calculation() {
        let view_model = ProductViewModel {
            id: "test".to_string(),
            name: "Test Product".to_string(),
            price: 1000,
            sale_price: Some(800),
            images: vec![],
            category: "test".to_string(),
            description: "Test".to_string(),
            material: None,
            dimensions: None,
            colors: vec![],
            is_on_sale: true,
            is_best_seller: false,
            is_quick_ship: false,
            is_sold_out: false,
            variants: vec![],
        };

        assert_eq!(view_model.calculate_discount_percentage(), Some(20));
        assert_eq!(view_model.calculate_savings_amount(), 200);
    }

    #[test]
    fn test_variant_creation() {
        let variant = VariantViewModel::new(
            "variant-1".to_string(),
            "Small".to_string(),
            1790,
            "Walnut".to_string(),
            Some("image.jpg".to_string()),
            true,
        );

        assert_eq!(variant.id, "variant-1");
        assert_eq!(variant.name, "Small");
        assert_eq!(variant.price, 1790);
        assert_eq!(variant.color, "Walnut");
        assert!(variant.is_available);
    }
}
