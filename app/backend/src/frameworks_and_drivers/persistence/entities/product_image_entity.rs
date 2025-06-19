use sqlx::FromRow;

/// 製品画像エンティティ
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct ProductImageEntity {
    pub id: i64,                           // INTEGER PK
    pub product_id: String,                // 外部キー (UUID)
    pub image_url: String,
    pub alt_text: Option<String>,
    pub display_order: i32,
    pub created_at: String,
    pub updated_at: String,
} 