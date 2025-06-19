use sqlx::FromRow;

/// タグエンティティ 
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct TagEntity {
    pub id: i64,                           // INTEGER PK
    pub slug: String,
    pub name: String,
    pub priority: i32,
    pub is_system: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// 製品タグ関連エンティティ
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct ProductTagEntity {
    pub id: i64,                           // INTEGER PK
    pub product_id: String,                // 外部キー (UUID)
    pub tag_id: i64,                       // 外部キー
    pub created_at: String,
} 