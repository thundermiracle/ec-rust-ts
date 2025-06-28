use crate::application::dto::{ProductDTO, VariantDTO};
use crate::presentation::products::responses::{VariantResponse, GetProductResponse};

/// GET /products/{id} API専用プレゼンター
/// Clean Architecture: Application層のProductDTOをInterface Adapter層のGetProductResponseに変換
/// GET /products/{id} 専用のプレゼンテーション層
pub struct GetProductPresenter;

impl GetProductPresenter {
    /// ProductDTOをGetProductResponseに変換（GET /products/{id}専用）
    /// 
    /// # Arguments
    /// * `product_dto` - Application層のProductDTO
    /// 
    /// # Returns
    /// * `GetProductResponse` - GET /products/{id} API専用レスポンス
    pub fn present(product_dto: ProductDTO) -> GetProductResponse {
        // variantsの変換
        let variants: Vec<VariantResponse> = product_dto
            .variants
            .into_iter()
            .map(Self::present_variant)
            .collect();

        GetProductResponse {
            id: product_dto.id,
            name: product_dto.name,
            images: product_dto.images,
            category: product_dto.category,
            description: product_dto.description,
            is_best_seller: product_dto.is_best_seller,
            is_quick_ship: product_dto.is_quick_ship,
            variants,
        }
    }

    /// VariantDTOをVariantResponseに変換
    /// 
    /// # Arguments
    /// * `variant_view_model` - Application層のVariantDTO
    /// 
    /// # Returns
    /// * `VariantResponse` - API応答用の形式に変換されたVariantResponse
    fn present_variant(variant_view_model: VariantDTO) -> VariantResponse {
        VariantResponse {
            id: variant_view_model.id,
            sku_code: variant_view_model.sku_code,
            name: variant_view_model.name,
            color: variant_view_model.color,
            material: variant_view_model.material,
            dimensions: variant_view_model.dimensions,
            price: variant_view_model.price,
            sale_price: variant_view_model.sale_price,
            // stock_quantity: variant_view_model.stock_quantity,
            display_order: variant_view_model.display_order,
            image: variant_view_model.image,
            is_on_sale: variant_view_model.is_on_sale,
            is_sold_out: variant_view_model.is_sold_out,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present_get_product_without_variants() {
        // Given: ProductDTOの作成
        let product_dto = ProductDTO::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            vec!["image1.jpg".to_string(), "image2.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            true,
            false,
            vec![],
        );

        // When: GetProductPresenterで変換
        let product_response = GetProductPresenter::present(product_dto);

        // Then: 正しく変換されている
        assert_eq!(product_response.id, "desk-walnut-1");
        assert_eq!(product_response.name, "Desk - Walnut");
        assert_eq!(product_response.images.len(), 2);
        assert_eq!(product_response.category, "desks");
        assert_eq!(product_response.description, "A beautiful walnut desk");
        assert!(product_response.is_best_seller);
        assert!(!product_response.is_quick_ship);
        assert!(product_response.variants.is_empty());
    }

    #[test]
    fn test_present_get_product_with_variants() {
        // Given: variantsを含むProductDTOの作成
        let variant1 = VariantDTO::new(
            "variant-1".to_string(),
            "SKU001".to_string(),
            "Small".to_string(),
            "Walnut".to_string(),
            "Wood".to_string(),
            "24x12x30".to_string(),
            1790,
            Some(1590),
            10,
            1,
            Some("variant1.jpg".to_string()),
        );

        let variant2 = VariantDTO::new(
            "variant-2".to_string(),
            "SKU002".to_string(),
            "Large".to_string(),
            "Black Oak".to_string(),
            "Wood".to_string(),
            "48x24x30".to_string(),
            2290,
            None,
            0,
            2,
            None,
        );

        let product_dto = ProductDTO::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            vec!["image1.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            false,
            true,
            vec![variant1, variant2],
        );

        // When: GetProductPresenterで変換
        let product_response = GetProductPresenter::present(product_dto);

        // Then: variantsも正しく変換されている
        assert_eq!(product_response.variants.len(), 2);
        
        let first_variant = &product_response.variants[0];
        assert_eq!(first_variant.id, "variant-1");
        assert_eq!(first_variant.sku_code, "SKU001");
        assert_eq!(first_variant.name, "Small");
        assert_eq!(first_variant.price, 1790);
        assert_eq!(first_variant.color, "Walnut");
        assert_eq!(first_variant.material, "Wood");
        assert_eq!(first_variant.dimensions, "24x12x30");
        assert_eq!(first_variant.sale_price, Some(1590));
        // assert_eq!(first_variant.stock_quantity, 10);
        assert_eq!(first_variant.display_order, 1);
        assert_eq!(first_variant.image, Some("variant1.jpg".to_string()));
        assert!(first_variant.is_on_sale);
        assert!(!first_variant.is_sold_out);

        let second_variant = &product_response.variants[1];
        assert_eq!(second_variant.id, "variant-2");
        assert_eq!(second_variant.sku_code, "SKU002");
        assert_eq!(second_variant.name, "Large");
        assert_eq!(second_variant.price, 2290);
        assert_eq!(second_variant.color, "Black Oak");
        assert_eq!(second_variant.material, "Wood");
        assert_eq!(second_variant.dimensions, "48x24x30");
        assert_eq!(second_variant.sale_price, None);
        // assert_eq!(second_variant.stock_quantity, 0);
        assert_eq!(second_variant.display_order, 2);
        assert_eq!(second_variant.image, None);
        assert!(!second_variant.is_on_sale);
        assert!(second_variant.is_sold_out);

        // プロダクトレベルのフラグの確認
        assert!(!product_response.is_best_seller);
        assert!(product_response.is_quick_ship);
    }

    #[test]
    fn test_present_variant() {
        // Given: VariantDTOの作成
        let variant_view_model = VariantDTO::new(
            "variant-test".to_string(),
            "SKU003".to_string(),
            "Medium".to_string(),
            "Walnut".to_string(),
            "Wood".to_string(),
            "36x18x30".to_string(),
            2000,
            Some(1800),
            5,
            1,
            Some("test.jpg".to_string()),
        );

        // When: VariantPresenterで変換
        let variant_response = GetProductPresenter::present_variant(variant_view_model);

        // Then: 正しく変換されている
        assert_eq!(variant_response.id, "variant-test");
        assert_eq!(variant_response.sku_code, "SKU003");
        assert_eq!(variant_response.name, "Medium");
        assert_eq!(variant_response.price, 2000);
        assert_eq!(variant_response.sale_price, Some(1800));
        assert_eq!(variant_response.color, "Walnut");
        assert_eq!(variant_response.material, "Wood");
        assert_eq!(variant_response.dimensions, "36x18x30");
        // assert_eq!(variant_response.stock_quantity, 5);
        assert_eq!(variant_response.display_order, 1);
        assert_eq!(variant_response.image, Some("test.jpg".to_string()));
        assert!(variant_response.is_on_sale);
        assert!(!variant_response.is_sold_out);
    }

    #[test]
    fn test_present_variant_without_image() {
        // Given: 画像なしのVariantDTOの作成
        let variant_view_model = VariantDTO::new(
            "variant-no-image".to_string(),
            "SKU004".to_string(),
            "XLarge".to_string(),
            "Oak".to_string(),
            "Wood".to_string(),
            "60x30x30".to_string(),
            3000,
            None,
            0,
            3,
            None,
        );

        // When: VariantPresenterで変換
        let variant_response = GetProductPresenter::present_variant(variant_view_model);

        // Then: 正しく変換されている
        assert_eq!(variant_response.id, "variant-no-image");
        assert_eq!(variant_response.sku_code, "SKU004");
        assert_eq!(variant_response.name, "XLarge");
        assert_eq!(variant_response.price, 3000);
        assert_eq!(variant_response.sale_price, None);
        assert_eq!(variant_response.color, "Oak");
        assert_eq!(variant_response.material, "Wood");
        assert_eq!(variant_response.dimensions, "60x30x30");
        // assert_eq!(variant_response.stock_quantity, 0);
        assert_eq!(variant_response.display_order, 3);
        assert_eq!(variant_response.image, None);
        assert!(!variant_response.is_on_sale);
        assert!(variant_response.is_sold_out);
    }
} 