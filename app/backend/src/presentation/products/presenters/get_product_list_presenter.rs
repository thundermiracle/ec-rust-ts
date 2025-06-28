use crate::application::dto::{ProductListDTO, ProductSummaryDTO};
use crate::presentation::products::responses::{
    GetProductListItemResponse, GetProductListResponse,
};

/// GET /products API専用プレゼンター
/// Clean Architecture: Interface Adapters層
/// GET /products API専用のアプリケーション層ViewModelをHTTPレスポンス用DTOに変換する
pub struct GetProductListPresenter;

impl GetProductListPresenter {
    /// ProductListDTOをGetProductListResponseに変換（GET /products専用）
    ///
    /// # Arguments
    /// * `view_model` - アプリケーション層から取得したProductListDTO
    ///
    /// # Returns
    /// HTTPレスポンス用のGetProductListResponse
    ///
    /// # Example
    /// ```rust
    /// let response = GetProductListPresenter::present_get_product_list(product_list_dto);
    /// ```
    pub fn present(view_model: ProductListDTO) -> GetProductListResponse {
        let products = view_model
            .products
            .into_iter()
            .map(Self::present_get_product_list_item)
            .collect();

        GetProductListResponse::new(
            products,
            view_model.total_count,
            view_model.page,
            view_model.per_page,
            view_model.has_next_page,
            view_model.has_previous_page,
        )
    }

    /// ProductSummaryDTOをGetProductListItemResponseに変換
    ///
    /// # Arguments
    /// * `summary` - 商品サマリーのViewModel
    ///
    /// # Returns
    /// HTTPレスポンス用のGetProductListItemResponse
    fn present_get_product_list_item(summary: ProductSummaryDTO) -> GetProductListItemResponse {
        // ViewModelから必要な情報を抽出
        let is_on_sale = summary.is_on_sale();
        let is_sold_out = summary.is_sold_out();
        
        GetProductListItemResponse::new(
            summary.id,
            summary.name,
            summary.base_price,
            summary.sale_price,
            summary.image.unwrap_or_default(),
            summary.category,
            summary.colors,
            Some(is_on_sale),
            Some(summary.is_best_seller),
            Some(summary.is_quick_ship),
            Some(is_sold_out),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{ProductListDTO, ProductSummaryDTO};

    /// GetProductListPresenterのテスト
    #[test]
    fn test_present_get_product_list() {
        // テストデータ作成
        let product_summary = ProductSummaryDTO::new(
            "product_1".to_string(),
            "テスト商品".to_string(),
            "家具".to_string(),
            15000,
            Some(12000),
            Some("https://example.com/image.jpg".to_string()),
            vec!["ブラック".to_string(), "ホワイト".to_string()],
            true,
            false,
            5,
        );

        let view_model = ProductListDTO {
            products: vec![product_summary],
            total_count: 1,
            page: 1,
            per_page: 10,
            has_next_page: false,
            has_previous_page: false,
        };

        // プレゼンテーション実行
        let response = GetProductListPresenter::present(view_model);

        // 検証
        assert_eq!(response.products.len(), 1);
        assert_eq!(response.products[0].id, "product_1");
        assert_eq!(response.products[0].name, "テスト商品");
        assert_eq!(response.products[0].category, "家具");
        assert_eq!(response.products[0].price, 15000);
        assert_eq!(response.products[0].sale_price, Some(12000));
        assert_eq!(response.products[0].is_on_sale, Some(true));
        assert_eq!(response.products[0].colors.len(), 2);
        assert_eq!(response.products[0].is_best_seller, Some(true));
        assert_eq!(response.products[0].is_quick_ship, Some(false));
        assert_eq!(response.products[0].is_sold_out, Some(false));
        assert_eq!(response.products[0].image, "https://example.com/image.jpg");

        assert_eq!(response.total_count, 1);
        assert_eq!(response.page, 1);
        assert_eq!(response.per_page, 10);
        assert!(!response.has_next_page);
        assert!(!response.has_previous_page);
    }

    /// 売り切れ商品のテスト
    #[test]
    fn test_present_sold_out_product() {
        let product_summary = ProductSummaryDTO::new(
            "product_2".to_string(),
            "売り切れ商品".to_string(),
            "家具".to_string(),
            20000,
            None,
            None,
            vec!["レッド".to_string()],
            false,
            true,
            0, // 在庫0
        );

        let view_model = ProductListDTO {
            products: vec![product_summary],
            total_count: 1,
            page: 1,
            per_page: 10,
            has_next_page: false,
            has_previous_page: false,
        };

        let response = GetProductListPresenter::present(view_model);

        assert_eq!(response.products[0].sale_price, None);
        assert_eq!(response.products[0].is_on_sale, Some(false));
        assert_eq!(response.products[0].is_sold_out, Some(true));
        assert_eq!(response.products[0].is_best_seller, Some(false));
        assert_eq!(response.products[0].is_quick_ship, Some(true));
        assert_eq!(response.products[0].image, ""); // 画像なし
    }

    /// 複数商品のテスト
    #[test]
    fn test_present_multiple_products() {
        let product1 = ProductSummaryDTO::new(
            "product_1".to_string(),
            "商品1".to_string(),
            "家具".to_string(),
            1000,
            Some(800),
            Some("https://example.com/1.jpg".to_string()),
            vec!["黒".to_string()],
            true,
            false,
            10,
        );

        let product2 = ProductSummaryDTO::new(
            "product_2".to_string(),
            "商品2".to_string(),
            "雑貨".to_string(),
            2000,
            None,
            None,
            vec!["白".to_string(), "青".to_string()],
            false,
            true,
            0,
        );

        let view_model = ProductListDTO {
            products: vec![product1, product2],
            total_count: 2,
            page: 1,
            per_page: 10,
            has_next_page: false,
            has_previous_page: false,
        };

        let response = GetProductListPresenter::present(view_model);

        assert_eq!(response.products.len(), 2);
        assert_eq!(response.total_count, 2);
    }
} 