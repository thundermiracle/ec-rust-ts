use sqlx::FromRow;

/// カテゴリエンティティ
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct CategoryEntity {
    pub id: String,                        // UUID PK
    pub name: String,
    pub slug: String,
    pub parent_id: Option<String>,         // 親カテゴリへの参照 (UUID)
    pub display_order: i32,
    pub created_at: String,
    pub updated_at: String,
} 