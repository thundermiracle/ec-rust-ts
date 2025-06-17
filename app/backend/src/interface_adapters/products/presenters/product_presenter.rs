use crate::application::queries::{
    CategoryQuery, ImageQuery, PriceQuery, ProductQuery, StatusQuery, StockQuery, VariantQuery,
};
use crate::interface_adapters::products::responses::{ProductResponse, VariantResponse};

/// Product用のプレゼンター
/// Clean Architecture: Interface Adapters層
pub struct ProductPresenter;

impl ProductPresenter {
    /// ProductQueryをAPI応答形式に変換
    pub fn present(product_query: ProductQuery) -> ProductResponse {
        // VariantQueryをVariantResponseに変換
        let variants: Vec<VariantResponse> = product_query
            .variants
            .into_iter()
            .map(VariantResponse::from)
            .collect();

        ProductResponse {
            id: product_query.id.to_string(),
            name: product_query.name,
            price: product_query.price.base_price,
            sale_price: product_query.price.sale_price,
            images: product_query.images.images,
            category: product_query.category.slug,
            description: product_query.description,
            material: product_query.material,
            dimensions: product_query.dimensions,
            colors: product_query.colors,
            is_on_sale: product_query.status.is_on_sale,
            is_best_seller: product_query.status.is_best_seller,
            is_quick_ship: product_query.status.is_quick_ship,
            is_sold_out: product_query.stock.is_sold_out,
            variants,
        }
    }


    /// 複数のProductをまとめて変換
    pub fn present_list(product_queries: Vec<ProductQuery>) -> Vec<ProductResponse> {
        product_queries.into_iter().map(Self::present).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::queries::VariantQuery;

    fn create_test_product_query() -> ProductQuery {
        ProductQuery {
            id: 1,
            name: "Test Product".to_string(),
            description: "Test Description".to_string(),
            material: Some("Cotton".to_string()),
            dimensions: Some("10x10x10cm".to_string()),
            price: PriceQuery {
                base_price: 1000,
                sale_price: Some(800),
                current_price: 800,
                discount_percentage: Some(20),
                savings_amount: 200,
            },
            category: CategoryQuery {
                id: "test-category-id".to_string(),
                name: "Test Category".to_string(),
                slug: "test-category".to_string(),
            },
            stock: StockQuery {
                quantity: 10,
                is_sold_out: false,
                is_available: true,
            },
            images: ImageQuery {
                images: vec!["image1.jpg".to_string()],
                main_image: Some("image1.jpg".to_string()),
            },
            status: StatusQuery {
                is_on_sale: true,
                is_best_seller: false,
                is_quick_ship: false,
                is_active: true,
            },
            colors: vec!["Red".to_string()],
            tags: vec!["sale".to_string()],
            variants: vec![],
        }
    }

    fn create_product_query_with_variants() -> ProductQuery {
        let mut product_query = create_test_product_query();
        product_query.id = 1;
        product_query.name = "Desk - Walnut".to_string();
        product_query.description =
            "Minimalist walnut desk with clean lines and modern design".to_string();
        product_query.material = Some("Walnut Wood".to_string());
        product_query.dimensions = Some("48\" x 24\" x 30\"".to_string());
        product_query.price = PriceQuery {
            base_price: 2290,
            sale_price: Some(1790),
            current_price: 1790,
            discount_percentage: Some(21), // calculated
            savings_amount: 500,
        };
        product_query.category.slug = "desks".to_string();
        product_query.images = ImageQuery {
            images: vec![
                "https://picsum.photos/id/100/800/800".to_string(),
                "https://picsum.photos/id/101/800/800".to_string(),
            ],
            main_image: Some("https://picsum.photos/id/100/800/800".to_string()),
        };
        product_query.colors = vec!["Walnut".to_string()];
        product_query.status.is_on_sale = true;
        product_query.status.is_best_seller = true;
        product_query.variants = vec![
            VariantQuery {
                id: "desk-walnut-small".to_string(),
                name: "Small".to_string(),
                price: 1790,
                color: "Walnut".to_string(),
                image: Some("https://picsum.photos/id/100/800/800".to_string()),
                is_available: true,
            },
            VariantQuery {
                id: "desk-walnut-large".to_string(),
                name: "Large".to_string(),
                price: 2290,
                color: "Walnut".to_string(),
                image: Some("https://picsum.photos/id/101/800/800".to_string()),
                is_available: true,
            },
        ];

        product_query
    }

    #[test]
    fn test_present_product_query() {
        let product_query = create_test_product_query();
        let response = ProductPresenter::present(product_query);

        assert_eq!(response.id, "1");
        assert_eq!(response.name, "Test Product");
        assert_eq!(response.price, 1000);
        assert_eq!(response.sale_price, Some(800));
        assert_eq!(response.category, "test-category");
        assert!(response.is_on_sale);
    }

    #[test]
    fn test_mockdata_compatible_json_output() {
        let product_query = create_product_query_with_variants();
        let response = ProductPresenter::present(product_query);

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