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
            price: product_view_model.price,
            sale_price: product_view_model.sale_price,
            images: product_view_model.images,
            category: product_view_model.category,
            description: product_view_model.description,
            material: product_view_model.material,
            dimensions: product_view_model.dimensions,
            colors: product_view_model.colors,
            is_on_sale: Some(product_view_model.is_on_sale).filter(|&val| val),
            is_best_seller: Some(product_view_model.is_best_seller).filter(|&val| val),
            is_quick_ship: Some(product_view_model.is_quick_ship).filter(|&val| val),
            is_sold_out: Some(product_view_model.is_sold_out).filter(|&val| val),
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
            name: variant_view_model.name,
            price: variant_view_model.price,
            color: variant_view_model.color,
            image: variant_view_model.image.unwrap_or_default(),
            is_available: variant_view_model.is_available,
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
            2290,
            Some(1790),
            vec!["image1.jpg".to_string(), "image2.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            Some("Walnut Wood".to_string()),
            Some("48\" x 24\" x 30\"".to_string()),
            vec!["Walnut".to_string(), "Black Oak".to_string()],
            true,
            true,
            false,
            false,
            vec![],
        );

        // When: ProductPresenterで変換
        let product_response = ProductPresenter::present(product_view_model);

        // Then: 正しく変換されている
        assert_eq!(product_response.id, "desk-walnut-1");
        assert_eq!(product_response.name, "Desk - Walnut");
        assert_eq!(product_response.price, 2290);
        assert_eq!(product_response.sale_price, Some(1790));
        assert_eq!(product_response.images.len(), 2);
        assert_eq!(product_response.category, "desks");
        assert_eq!(product_response.description, "A beautiful walnut desk");
        assert_eq!(product_response.material, Some("Walnut Wood".to_string()));
        assert_eq!(product_response.dimensions, Some("48\" x 24\" x 30\"".to_string()));
        assert_eq!(product_response.colors.len(), 2);
        assert_eq!(product_response.is_on_sale, Some(true));
        assert_eq!(product_response.is_best_seller, Some(true));
        assert_eq!(product_response.is_quick_ship, None); // false値はNoneになる
        assert_eq!(product_response.is_sold_out, None);   // false値はNoneになる
        assert!(product_response.variants.is_empty());
    }

    #[test]
    fn test_present_product_with_variants() {
        // Given: variantsを含むProductViewModelの作成
        let variant1 = VariantViewModel::new(
            "variant-1".to_string(),
            "Small".to_string(),
            1790,
            "Walnut".to_string(),
            Some("variant1.jpg".to_string()),
            true,
        );

        let variant2 = VariantViewModel::new(
            "variant-2".to_string(),
            "Large".to_string(),
            2290,
            "Black Oak".to_string(),
            None,
            false,
        );

        let product_view_model = ProductViewModel::new(
            "desk-walnut-1".to_string(),
            "Desk - Walnut".to_string(),
            1790,
            None,
            vec!["image1.jpg".to_string()],
            "desks".to_string(),
            "A beautiful walnut desk".to_string(),
            Some("Walnut Wood".to_string()),
            Some("48\" x 24\" x 30\"".to_string()),
            vec!["Walnut".to_string(), "Black Oak".to_string()],
            false,
            false,
            true,
            false,
            vec![variant1, variant2],
        );

        // When: ProductPresenterで変換
        let product_response = ProductPresenter::present(product_view_model);

        // Then: variantsも正しく変換されている
        assert_eq!(product_response.variants.len(), 2);
        
        let first_variant = &product_response.variants[0];
        assert_eq!(first_variant.id, "variant-1");
        assert_eq!(first_variant.name, "Small");
        assert_eq!(first_variant.price, 1790);
        assert_eq!(first_variant.color, "Walnut");
        assert_eq!(first_variant.image, "variant1.jpg");
        assert!(first_variant.is_available);

        let second_variant = &product_response.variants[1];
        assert_eq!(second_variant.id, "variant-2");
        assert_eq!(second_variant.name, "Large");
        assert_eq!(second_variant.price, 2290);
        assert_eq!(second_variant.color, "Black Oak");
        assert_eq!(second_variant.image, ""); // None -> 空文字列
        assert!(!second_variant.is_available);

        // フラグの確認
        assert_eq!(product_response.is_on_sale, None);     // false値はNoneになる
        assert_eq!(product_response.is_best_seller, None); // false値はNoneになる
        assert_eq!(product_response.is_quick_ship, Some(true));
        assert_eq!(product_response.is_sold_out, None);   // false値はNoneになる
    }

    #[test]
    fn test_present_variant() {
        // Given: VariantViewModelの作成
        let variant_view_model = VariantViewModel::new(
            "variant-test".to_string(),
            "Medium".to_string(),
            2000,
            "Walnut".to_string(),
            Some("test.jpg".to_string()),
            true,
        );

        // When: VariantPresenterで変換
        let variant_response = ProductPresenter::present_variant(variant_view_model);

        // Then: 正しく変換されている
        assert_eq!(variant_response.id, "variant-test");
        assert_eq!(variant_response.name, "Medium");
        assert_eq!(variant_response.price, 2000);
        assert_eq!(variant_response.color, "Walnut");
        assert_eq!(variant_response.image, "test.jpg");
        assert!(variant_response.is_available);
    }

    #[test]
    fn test_present_variant_without_image() {
        // Given: imageがNoneのVariantViewModel
        let variant_view_model = VariantViewModel::new(
            "variant-no-image".to_string(),
            "No Image".to_string(),
            1500,
            "Black Oak".to_string(),
            None,
            false,
        );

        // When: VariantPresenterで変換
        let variant_response = ProductPresenter::present_variant(variant_view_model);

        // Then: imageは空文字列になる
        assert_eq!(variant_response.id, "variant-no-image");
        assert_eq!(variant_response.name, "No Image");
        assert_eq!(variant_response.price, 1500);
        assert_eq!(variant_response.color, "Black Oak");
        assert_eq!(variant_response.image, ""); // None -> 空文字列
        assert!(!variant_response.is_available);
    }
}
