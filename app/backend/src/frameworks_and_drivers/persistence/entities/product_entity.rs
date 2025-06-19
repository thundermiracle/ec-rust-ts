use sqlx::FromRow;

/// データベースエンティティ - 製品基本情報
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct ProductEntity {
    pub id: String,                        // TEXT型 (UUID)
    pub name: String,
    pub description: String,
    pub category_id: String,               // TEXT型 (UUID)
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub is_active: bool,                   // DBの available と同等
    pub created_at: String,
    pub updated_at: String,
}