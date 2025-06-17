use crate::application::error::RepositoryError;
use crate::application::queries::ProductQuery;

/// 商品表示用サービスインターフェース
/// Clean Architecture: Application層でのView専用Repository
#[async_trait::async_trait]
pub trait ProductQueryService {
    /// IDで詳細商品情報を取得
    async fn get_detailed_product(&self, id: u32) -> Result<Option<ProductQuery>, RepositoryError>;
    
    /// 全ての詳細商品情報を取得
    async fn get_all_detailed_products(&self) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// カテゴリー別詳細商品情報を取得
    async fn get_detailed_products_by_category(&self, category_id: u32) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// ベストセラー商品を取得
    async fn get_best_seller_products(&self) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// セール商品を取得
    async fn get_on_sale_products(&self) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// 迅速配送商品を取得
    async fn get_quick_ship_products(&self) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// 商品検索（名前で部分一致）
    async fn search_products(&self, query: &str) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// ページネーション付き商品取得
    async fn get_products_paginated(
        &self,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ProductQuery>, u64), RepositoryError>; // (products, total_count)
}

/// 商品フィルター条件
#[derive(Debug, Clone, Default)]
pub struct ProductFilter {
    pub category_id: Option<u32>,
    pub color_names: Vec<String>,
    pub tags: Vec<String>,
    pub price_min: Option<u32>,
    pub price_max: Option<u32>,
    pub is_on_sale: Option<bool>,
    pub is_best_seller: Option<bool>,
    pub is_quick_ship: Option<bool>,
    pub is_available_only: bool,
}

/// 商品ソート条件
#[derive(Debug, Clone)]
pub enum ProductSort {
    NameAsc,
    NameDesc,
    PriceAsc,
    PriceDesc,
    CreatedAtAsc,
    CreatedAtDesc,
    BestSeller,
    OnSale,
}

impl Default for ProductSort {
    fn default() -> Self {
        ProductSort::CreatedAtDesc
    }
}

/// 高度な商品表示用サービスインターフェース（拡張版）
#[async_trait::async_trait]
pub trait AdvancedProductQueryService: ProductQueryService {
    /// フィルター・ソート・ページネーション付き商品取得
    async fn get_products_advanced(
        &self,
        filter: &ProductFilter,
        sort: &ProductSort,
        page: u32,
        per_page: u32,
    ) -> Result<(Vec<ProductQuery>, u64), RepositoryError>;
    
    /// 関連商品を取得（同カテゴリー、類似価格帯）
    async fn get_related_products(&self, product_id: u32, limit: u32) -> Result<Vec<ProductQuery>, RepositoryError>;
    
    /// おすすめ商品を取得
    async fn get_recommended_products(&self, limit: u32) -> Result<Vec<ProductQuery>, RepositoryError>;
}

impl ProductFilter {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_category(mut self, category_id: u32) -> Self {
        self.category_id = Some(category_id);
        self
    }
    
    pub fn with_colors(mut self, colors: Vec<String>) -> Self {
        self.color_names = colors;
        self
    }
    
    pub fn with_price_range(mut self, min: Option<u32>, max: Option<u32>) -> Self {
        self.price_min = min;
        self.price_max = max;
        self
    }
    
    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
    
    pub fn on_sale_only(mut self) -> Self {
        self.is_on_sale = Some(true);
        self
    }
    
    pub fn best_sellers_only(mut self) -> Self {
        self.is_best_seller = Some(true);
        self
    }
    
    pub fn quick_ship_only(mut self) -> Self {
        self.is_quick_ship = Some(true);
        self
    }
    
    pub fn available_only(mut self) -> Self {
        self.is_available_only = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_product_filter_builder() {
        let filter = ProductFilter::new()
            .with_category(1)
            .with_colors(vec!["Red".to_string(), "Blue".to_string()])
            .with_price_range(Some(1000), Some(5000))
            .on_sale_only()
            .available_only();
        
        assert_eq!(filter.category_id, Some(1));
        assert_eq!(filter.color_names, vec!["Red", "Blue"]);
        assert_eq!(filter.price_min, Some(1000));
        assert_eq!(filter.price_max, Some(5000));
        assert_eq!(filter.is_on_sale, Some(true));
        assert!(filter.is_available_only);
    }
} 