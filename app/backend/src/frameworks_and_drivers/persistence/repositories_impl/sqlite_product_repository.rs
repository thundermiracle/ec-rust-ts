use crate::domain::models::Product;
use crate::frameworks_and_drivers::database::db::get_db;
use crate::frameworks_and_drivers::persistence::entities::ProductEntity;
use crate::application::repositories::ProductRepository;
use crate::application::error::RepositoryError;

/// SQLite商品リポジトリ実装 - 新しい正規化スキーマ対応
/// Clean Architecture: Frameworks & Drivers層のデータアクセス実装
pub struct SqliteProductRepository;

impl SqliteProductRepository {
    pub fn new() -> Self {
        Self {}
    }
    

}

#[async_trait::async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let entities = sqlx::query_as::<_, ProductEntity>(
            r#"
            SELECT id, name, description, material, dimensions, 
                   base_price, sale_price, category_id, quantity,
                   is_active, is_best_seller, is_quick_ship,
                   created_at, updated_at
            FROM products 
            WHERE is_active = TRUE
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        let products = entities
            .into_iter()
            .map(|entity| entity.to_domain())
            .collect::<Result<Vec<Product>, _>>()
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        Ok(products)
    }

    async fn find_by_id(&self, id: u32) -> Result<Option<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let entity = sqlx::query_as::<_, ProductEntity>(
            r#"
            SELECT id, name, description, material, dimensions, 
                   base_price, sale_price, category_id, quantity,
                   is_active, is_best_seller, is_quick_ship,
                   created_at, updated_at
            FROM products 
            WHERE id = ? AND is_active = TRUE
            "#
        )
        .bind(id as i64)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        match entity {
            Some(entity) => {
                let product = entity.to_domain()
                    .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
                Ok(Some(product))
            },
            None => Ok(None),
        }
    }

    async fn save(&self, product: Product) -> Result<(), RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let entity = ProductEntity::from_domain(&product);
        
        // 既存のプロダクトを検索
        let existing = sqlx::query_as::<_, ProductEntity>(
            "SELECT * FROM products WHERE id = ?"
        )
        .bind(entity.id)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        match existing {
            // 更新
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE products 
                    SET name = ?, description = ?, material = ?, dimensions = ?,
                        base_price = ?, sale_price = ?, category_id = ?, quantity = ?,
                        is_active = ?, is_best_seller = ?, is_quick_ship = ?,
                        updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(&entity.name)
                .bind(&entity.description)
                .bind(&entity.material)
                .bind(&entity.dimensions)
                .bind(entity.base_price)
                .bind(entity.sale_price)
                .bind(entity.category_id)
                .bind(entity.quantity)
                .bind(entity.is_active)
                .bind(entity.is_best_seller)
                .bind(entity.is_quick_ship)
                .bind(&entity.updated_at)
                .bind(entity.id)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            },
            // 新規作成
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO products (
                        name, description, material, dimensions,
                        base_price, sale_price, category_id, quantity,
                        is_active, is_best_seller, is_quick_ship,
                        created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&entity.name)
                .bind(&entity.description)
                .bind(&entity.material)
                .bind(&entity.dimensions)
                .bind(entity.base_price)
                .bind(entity.sale_price)
                .bind(entity.category_id)
                .bind(entity.quantity)
                .bind(entity.is_active)
                .bind(entity.is_best_seller)
                .bind(entity.is_quick_ship)
                .bind(&entity.created_at)
                .bind(&entity.updated_at)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            }
        }
        
        Ok(())
    }
}
