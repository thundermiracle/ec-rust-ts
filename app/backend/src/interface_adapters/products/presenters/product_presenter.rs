use crate::application::queries::{ProductDetail, ProductSummary, VariantDetail};
use serde::Serialize;

/// Product用のプレゼンター
/// Clean Architecture: Interface Adapters層
pub struct ProductPresenter;

impl ProductPresenter {
    /// ProductDetailをAPI応答形式に変換
    pub fn present(product_detail: ProductDetail) -> ProductResponse {
        // VariantDetailをVariantResponseに変換
        let variants: Vec<VariantResponse> = product_detail.variants
            .into_iter()
            .map(VariantResponse::from)
            .collect();
            
        ProductResponse {
            id: product_detail.id.to_string(), // mockDataに合わせて文字列に
            name: product_detail.name,
            price: product_detail.base_price,
            sale_price: product_detail.sale_price,
            images: product_detail.images,
            category: product_detail.category_slug, // mockDataに合わせて文字列に
            description: product_detail.description,
            material: product_detail.material,
            dimensions: product_detail.dimensions,
            colors: product_detail.colors,
            is_on_sale: product_detail.is_on_sale,
            is_best_seller: product_detail.is_best_seller,
            is_quick_ship: product_detail.is_quick_ship,
            is_sold_out: product_detail.is_sold_out,
            variants,
        }
    }

    /// ProductSummaryからProductResponseを作成（簡易版）
    pub fn present_summary(product_summary: ProductSummary) -> ProductResponse {
        ProductResponse {
            id: product_summary.id.to_string(),
            name: product_summary.name,
            price: product_summary.price,
            sale_price: None,
            images: vec![],
            category: "unknown".to_string(),
            description: product_summary.description,
            material: None,
            dimensions: None,
            colors: vec![],
            is_on_sale: false,
            is_best_seller: false,
            is_quick_ship: false,
            is_sold_out: product_summary.quantity == 0,
            variants: vec![],
        }
    }
    
    /// 複数のProductをまとめて変換
    pub fn present_list(product_views: Vec<ProductDetail>) -> Vec<ProductResponse> {
        product_views.into_iter().map(Self::present).collect()
    }
}

/// ProductSummaryからProductResponseへの変換実装
impl From<ProductSummary> for ProductResponse {
    fn from(summary: ProductSummary) -> Self {
        ProductPresenter::present_summary(summary)
    }
}

/// VariantDetailからVariantResponseへの変換実装
impl From<VariantDetail> for VariantResponse {
    fn from(detail: VariantDetail) -> Self {
        VariantResponse {
            id: detail.id,
            name: detail.name,
            price: detail.price,
            color: detail.color,
            image: detail.image,
            is_available: detail.is_available,
        }
    }
}

/// API応答用のProduct構造体（mockData.tsに合わせた構造）
#[derive(Debug, Serialize)]
pub struct ProductResponse {
    pub id: String,                    // mockDataに合わせて文字列
    pub name: String,
    pub price: u32,                    // base_priceからpriceに変更
    #[serde(rename = "salePrice", skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<u32>,       // salePrice形式
    pub images: Vec<String>,
    pub category: String,              // カテゴリーは文字列（slug）
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<String>,
    pub colors: Vec<String>,
    #[serde(rename = "isOnSale")]
    pub is_on_sale: bool,
    #[serde(rename = "isBestSeller")]
    pub is_best_seller: bool,
    #[serde(rename = "isQuickShip")]
    pub is_quick_ship: bool,
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantResponse>,
}

/// Product Variant応答構造体（mockData.tsのvariant構造に合わせる）
#[derive(Debug, Serialize)]
pub struct VariantResponse {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<String>,
    #[serde(rename = "isAvailable")]
    pub is_available: bool,
}

/// カテゴリー応答構造体（将来のために残しておく）
#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,                    // 文字列IDに変更
    pub name: String,
    pub slug: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::queries::VariantDetail;

    fn create_test_product_detail() -> ProductDetail {
        ProductDetail {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            material: Some("Cotton".to_string()),
            dimensions: Some("10x10x10cm".to_string()),
            base_price: 1000,
            sale_price: Some(800),
            current_price: 800,
            discount_percentage: Some(20),
            savings_amount: 200,
            category_id: 1,
            category_name: "Test Category".to_string(),
            category_slug: "test-category".to_string(),
            images: vec!["image1.jpg".to_string()],
            main_image: Some("image1.jpg".to_string()),
            colors: vec!["Red".to_string()],
            tags: vec!["sale".to_string()],
            is_on_sale: true,
            is_best_seller: false,
            is_quick_ship: false,
            is_sold_out: false,
            is_active: true,
            quantity: 10,
            is_available: true,
            variants: vec![],
            created_at: "2023-01-01T00:00:00Z".to_string(),
            updated_at: "2023-01-01T00:00:00Z".to_string(),
        }
    }

    fn create_product_detail_with_variants() -> ProductDetail {
        let mut product_detail = create_test_product_detail();
        product_detail.id = 1;
        product_detail.name = "Desk - Walnut".to_string();
        product_detail.description = "Minimalist walnut desk with clean lines and modern design".to_string();
        product_detail.material = Some("Walnut Wood".to_string());
        product_detail.dimensions = Some("48\" x 24\" x 30\"".to_string());
        product_detail.base_price = 2290;
        product_detail.sale_price = Some(1790);
        product_detail.category_slug = "desks".to_string();
        product_detail.images = vec![
            "https://picsum.photos/id/100/800/800".to_string(),
            "https://picsum.photos/id/101/800/800".to_string(),
        ];
        product_detail.colors = vec!["Walnut".to_string()];
        product_detail.is_on_sale = true;
        product_detail.is_best_seller = true;
        product_detail.variants = vec![
            VariantDetail {
                id: "desk-walnut-small".to_string(),
                name: "Small".to_string(),
                price: 1790,
                color: "Walnut".to_string(),
                image: Some("https://picsum.photos/id/100/800/800".to_string()),
                is_available: true,
            },
            VariantDetail {
                id: "desk-walnut-large".to_string(),
                name: "Large".to_string(),
                price: 2290,
                color: "Walnut".to_string(),
                image: Some("https://picsum.photos/id/101/800/800".to_string()),
                is_available: true,
            },
        ];
        
        product_detail
    }

    #[test]
    fn test_present_product_detail() {
        let product_detail = create_test_product_detail();
        let response = ProductPresenter::present(product_detail);
        
        assert_eq!(response.id, "1");
        assert_eq!(response.name, "Test Product");
        assert_eq!(response.price, 1000);
        assert_eq!(response.sale_price, Some(800));
        assert_eq!(response.category, "test-category");
        assert!(response.is_on_sale);
    }

    #[test]
    fn test_present_summary() {
        let summary = ProductSummary {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            price: 1000,
            quantity: 5,
        };
        
        let response = ProductPresenter::present_summary(summary);
        
        assert_eq!(response.id, "1");
        assert_eq!(response.name, "Test Product");
        assert_eq!(response.price, 1000);
        assert_eq!(response.category, "unknown");
        assert!(!response.is_on_sale);
    }

    #[test]
    fn test_mockdata_compatible_json_output() {
        let product_detail = create_product_detail_with_variants();
        let response = ProductPresenter::present(product_detail);
        
        // JSON出力をテスト
        let json_output = serde_json::to_string_pretty(&response).unwrap();
        println!("Generated ProductResponse JSON (compatible with mockData.ts):");
        println!("{}", json_output);
        
        // キー構造をテスト
        assert_eq!(response.id, "1");
        assert_eq!(response.name, "Desk - Walnut");
        assert_eq!(response.price, 2290);
        assert_eq!(response.sale_price, Some(1790));
        assert_eq!(response.category, "desks");
        assert_eq!(response.variants.len(), 2);
        
        // バリアント構造をテスト
        let small_variant = &response.variants[0];
        assert_eq!(small_variant.id, "desk-walnut-small");
        assert_eq!(small_variant.name, "Small");
        assert_eq!(small_variant.price, 1790);
        assert_eq!(small_variant.color, "Walnut");
        assert!(small_variant.is_available);
        
        let large_variant = &response.variants[1];
        assert_eq!(large_variant.id, "desk-walnut-large");
        assert_eq!(large_variant.name, "Large");
        assert_eq!(large_variant.price, 2290);
        assert_eq!(large_variant.color, "Walnut");
        assert!(large_variant.is_available);
    }
} 