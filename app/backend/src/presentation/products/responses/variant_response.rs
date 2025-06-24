use serde::Serialize;
use utoipa::ToSchema;

/// API応答用のVariant構造体（VariantDTOに合わせた構造）
#[derive(Debug, Serialize, ToSchema)]
pub struct VariantResponse {
    /// バリエーションID
    pub id: String,
    /// SKU商品コード
    #[serde(rename = "skuCode")]
    pub sku_code: String,
    /// バリエーション名
    pub name: String,
    /// 色
    pub color: String,
    /// 素材
    pub material: String,
    /// サイズ
    pub dimensions: String,
    /// 価格（円）
    pub price: u32,
    /// セール価格（円）
    #[serde(rename = "salePrice", skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub sale_price: Option<u32>,
    // #[serde(rename = "stockQuantity")]
    // pub stock_quantity: u32,
    /// 表示順
    #[serde(rename = "displayOrder")]
    pub display_order: u32,
    /// バリエーション画像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub image: Option<String>,
    /// セール中かどうか
    #[serde(rename = "isOnSale")]
    pub is_on_sale: bool,
    /// 品切れかどうか
    #[serde(rename = "isSoldOut")]
    pub is_sold_out: bool,
} 