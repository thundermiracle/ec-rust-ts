use serde::Serialize;
use utoipa::ToSchema;
use super::variant_response::VariantResponse;

/// GET /products/{id} API専用の商品詳細レスポンス構造体
#[derive(Debug, Serialize, ToSchema)]
pub struct GetProductResponse {
    /// 商品ID
    pub id: String,
    /// 商品名
    pub name: String,
    /// 商品画像URL一覧
    pub images: Vec<String>,
    /// カテゴリー名
    pub category: String,
    /// 商品説明
    pub description: String,
    /// ベストセラー商品かどうか
    #[serde(rename = "isBestSeller")]
    pub is_best_seller: bool,
    /// 即配送可能かどうか
    #[serde(rename = "isQuickShip")]
    pub is_quick_ship: bool,
    /// バリエーション一覧
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<VariantResponse>,
} 