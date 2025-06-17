use sqlx::FromRow;
use crate::domain::models::{Product, Money};
use crate::domain::error::DomainError;

/// データベースエンティティ - 新しい正規化スキーマに対応
/// Clean Architecture: Frameworks & Drivers層のデータ表現
#[derive(Debug, FromRow)]
pub struct ProductEntity {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    pub base_price: i64,                    // JPY in smallest unit
    pub sale_price: Option<i64>,            // JPY in smallest unit
    pub category_id: i64,
    pub quantity: i64,
    pub is_active: bool,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl ProductEntity {
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(product: &Product) -> Self {
        Self {
            id: product.id as i64,
            name: product.name.clone(),
            description: product.description.clone(),
            material: product.material.clone(),
            dimensions: product.dimensions.clone(),
            base_price: product.base_price.yen() as i64,
            sale_price: product.sale_price.map(|price| price.yen() as i64),
            category_id: 1, // TODO: カテゴリーIDの適切な処理
            quantity: product.quantity as i64,
            is_active: product.is_available,
            is_best_seller: product.is_best_seller,
            is_quick_ship: product.is_quick_ship,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }
    }
    
    /// エンティティからドメインモデルへの変換
    pub fn to_domain(self) -> Result<Product, DomainError> {
        Product::new_simple(
            self.id as u32,
            self.name,
            self.description,
            self.quantity as u32,
            Money::from_yen(self.base_price as u32),
        )
    }
}