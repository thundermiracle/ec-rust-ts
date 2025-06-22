use crate::application::viewmodels::{ProductViewModel, VariantViewModel};
use crate::interface_adapters::products::responses::{ProductResponse, VariantResponse};

/// ProductPresenter
/// Clean Architecture: Application層のProductViewModelをInterface Adapter層のProductResponseに変換
/// プレゼンテーション層への出力形式を担当
pub struct ProductPresenter;

impl ProductPresenter {
    /// ProductViewModelをProductResponseに変換
    /// 
    /// # Arguments
    /// * `product_view_model` - Application層のProductViewModel
    /// 
    /// # Returns
    /// * `ProductResponse` - API応答用の形式に変換されたProductResponse
    pub fn present(product_view_model: ProductViewModel) -> ProductResponse {
        // variantsの変換
        let variants: Vec<VariantResponse> = product_view_model
            .variants
            .into_iter()
            .map(Self::present_variant)
            .collect();

        ProductResponse {
            id: product_view_model.id,
            name: product_view_model.name,
            images: product_view_model.images,
            category: product_view_model.category,
            description: product_view_model.description,
            is_best_seller: product_view_model.is_best_seller,
            is_quick_ship: product_view_model.is_quick_ship,
            variants,
        }
    }

    /// VariantViewModelをVariantResponseに変換
    /// 
    /// # Arguments
    /// * `variant_view_model` - Application層のVariantViewModel
    /// 
    /// # Returns
    /// * `VariantResponse` - API応答用の形式に変換されたVariantResponse
    fn present_variant(variant_view_model: VariantViewModel) -> VariantResponse {
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
    fn test_present_product_without_variants() {
        // Given: ProductViewModelの作成
        let product_view_model = ProductViewModel::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            vec!["image1.jpg".to_string(), "image2.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            true,
            false,
            vec![],
        );

        // When: ProductPresenterで変換
        let product_response = ProductPresenter::present(product_view_model);

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
    fn test_present_product_with_variants() {
        // Given: variantsを含むProductViewModelの作成
        let variant1 = VariantViewModel::new(
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

        let variant2 = VariantViewModel::new(
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

        let product_view_model = ProductViewModel::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            vec!["image1.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            false,
            true,
            vec![variant1, variant2],
        );

        // When: ProductPresenterで変換
        let product_response = ProductPresenter::present(product_view_model);

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
        // Given: VariantViewModelの作成
        let variant_view_model = VariantViewModel::new(
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
        let variant_response = ProductPresenter::present_variant(variant_view_model);

        // Then: 正しく変換されている
        assert_eq!(variant_response.id, "variant-test");
        assert_eq!(variant_response.sku_code, "SKU003");
        assert_eq!(variant_response.name, "Medium");
        assert_eq!(variant_response.price, 2000);
        assert_eq!(variant_response.color, "Walnut");
        assert_eq!(variant_response.material, "Wood");
        assert_eq!(variant_response.dimensions, "36x18x30");
        assert_eq!(variant_response.sale_price, Some(1800));
        // assert_eq!(variant_response.stock_quantity, 5);
        assert_eq!(variant_response.display_order, 1);
        assert_eq!(variant_response.image, Some("test.jpg".to_string()));
        assert!(variant_response.is_on_sale);
        assert!(!variant_response.is_sold_out);
    }

    #[test]
    fn test_present_variant_without_image() {
        // Given: imageがNoneのVariantViewModel
        let variant_view_model = VariantViewModel::new(
            "variant-no-image".to_string(),
            "SKU004".to_string(),
            "No Image".to_string(),
            "Black Oak".to_string(),
            "Wood".to_string(),
            "20x10x25".to_string(),
            1500,
            None,
            0,
            1,
            None,
        );

        // When: VariantPresenterで変換
        let variant_response = ProductPresenter::present_variant(variant_view_model);

        // Then: imageはNoneのまま
        assert_eq!(variant_response.id, "variant-no-image");
        assert_eq!(variant_response.sku_code, "SKU004");
        assert_eq!(variant_response.name, "No Image");
        assert_eq!(variant_response.price, 1500);
        assert_eq!(variant_response.color, "Black Oak");
        assert_eq!(variant_response.material, "Wood");
        assert_eq!(variant_response.dimensions, "20x10x25");
        assert_eq!(variant_response.sale_price, None);
        // assert_eq!(variant_response.stock_quantity, 0);
        assert_eq!(variant_response.display_order, 1);
        assert_eq!(variant_response.image, None);
        assert!(!variant_response.is_on_sale);
        assert!(variant_response.is_sold_out);
    }
}
