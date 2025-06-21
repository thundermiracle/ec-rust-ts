use serde::{Deserialize, Serialize};

/// 商品一覧のHTTPレスポンス用DTO
/// Clean Architecture: Interface Adapters層
/// TypeScriptのProduct型と整合性を取った構造
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListResponse {
    /// 商品一覧
    pub products: Vec<ProductListItemResponse>,
    /// 総件数
    pub total_count: u32,
    /// 現在のページ番号
    pub page: u32,
    /// 1ページあたりの件数
    pub per_page: u32,
    /// 次のページがあるかどうか
    pub has_next_page: bool,
    /// 前のページがあるかどうか
    pub has_previous_page: bool,
}

/// 商品のHTTPレスポンス用DTO
/// TypeScriptのProduct型に対応
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductListItemResponse {
    /// 商品ID
    pub id: String,
    /// 商品名
    pub name: String,
    /// 基本価格（円）
    pub price: u32,
    /// セール価格（円）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_price: Option<u32>,
    /// 商品画像URL一覧
    pub image: String,
    /// カテゴリー名
    pub category: String,
    /// 利用可能な色一覧
    pub colors: Vec<String>,
    /// セール中かどうか
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_on_sale: Option<bool>,
    /// ベストセラー商品かどうか
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_best_seller: Option<bool>,
    /// 即配送可能かどうか
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_quick_ship: Option<bool>,
    /// 売り切れかどうか
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_sold_out: Option<bool>,
}

impl ProductListResponse {
    /// 新しいProductListResponseを作成
    pub fn new(
        products: Vec<ProductListItemResponse>,
        total_count: u32,
        page: u32,
        per_page: u32,
        has_next_page: bool,
        has_previous_page: bool,
    ) -> Self {
        Self {
            products,
            total_count,
            page,
            per_page,
            has_next_page,
            has_previous_page,
        }
    }
}

impl ProductListItemResponse {
    /// 新しいProductListItemResponseを作成
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: String,
        name: String,
        price: u32,
        sale_price: Option<u32>,
        image: String,
        category: String,
        colors: Vec<String>,
        is_on_sale: Option<bool>,
        is_best_seller: Option<bool>,
        is_quick_ship: Option<bool>,
        is_sold_out: Option<bool>,
    ) -> Self {
        Self {
            id,
            name,
            price,
            sale_price,
            category,
            image,
            colors,
            is_on_sale,
            is_best_seller,
            is_quick_ship,
            is_sold_out,
        }
    }
}
