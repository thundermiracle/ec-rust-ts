use sqlx::FromRow;

/// カラーエンティティ
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct ColorEntity {
    pub id: i64,                           // INTEGER PK
    pub name: String,
    pub hex: String,
    pub created_at: String,
    pub updated_at: String,
} 