use sqlx::FromRow;

/// SKUエンティティ - 製品バリアント
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct SKUEntity {
    pub id: String,                        // TEXT型 (UUID)
    pub product_id: String,                // 外部キー
    pub sku_code: String,
    pub name: String,
    pub color_id: Option<i64>,             // INTEGER型
    pub dimensions: Option<String>,
    pub material: Option<String>,
    pub base_price: i64,                   // JPY in smallest unit
    pub sale_price: Option<i64>,           // JPY in smallest unit
    pub stock_quantity: i64,
    pub reserved_quantity: i64,
    pub low_stock_threshold: Option<i64>,
    pub display_order: i64,
    pub image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
} 