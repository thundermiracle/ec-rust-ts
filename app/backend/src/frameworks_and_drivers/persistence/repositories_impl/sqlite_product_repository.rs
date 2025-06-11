use sqlx::Row;
use chrono::Utc;

use crate::domain::models::Product;
use crate::frameworks_and_drivers::database::db::get_db;
use crate::frameworks_and_drivers::persistence::entities::ProductEntity;
use crate::application::repositories::ProductRepository;
use crate::application::error::RepositoryError;

pub struct SqliteProductRepository;

impl SqliteProductRepository {
    pub fn new() -> Self {
        Self {}
    }
    
    // エンティティからドメインモデルへのマッピング
    fn entity_to_domain(entity: ProductEntity) -> Product {
        Product::new(
            entity.id,
            entity.name,
            entity.price,
            entity.description,
            entity.quantity,
        )
    }
}

#[async_trait::async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let rows = sqlx::query("SELECT * FROM products")
            .fetch_all(pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        let products = rows
            .iter()
            .map(|row| {
                let entity = ProductEntity {
                    id: row.get("id"),
                    name: row.get("name"),
                    price: row.get("price"),
                    description: row.get("description"),
                    quantity: row.get("quantity"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                
                Self::entity_to_domain(entity)
            })
            .collect::<Vec<Product>>();
        
        Ok(products)
    }

    async fn find_by_id(&self, id: u32) -> Result<Option<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let row = sqlx::query("SELECT * FROM products WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        match row {
            Some(row) => {
                let entity = ProductEntity {
                    id: row.get("id"),
                    name: row.get("name"),
                    price: row.get("price"),
                    description: row.get("description"),
                    quantity: row.get("quantity"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                
                Ok(Some(Self::entity_to_domain(entity)))
            },
            None => Ok(None),
        }
    }

    async fn save(&self, product: Product) -> Result<(), RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let now = Utc::now().to_rfc3339();
        
        // 既存のプロダクトを検索
        let existing = sqlx::query("SELECT * FROM products WHERE id = ?")
            .bind(product.id)
            .fetch_optional(pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        match existing {
            // 更新
            Some(_) => {
                sqlx::query(
                    "UPDATE products SET name = ?, price = ?, description = ?, quantity = ?, updated_at = ? WHERE id = ?"
                )
                .bind(&product.name)
                .bind(product.price)
                .bind(&product.description)
                .bind(product.quantity)
                .bind(&now)
                .bind(product.id)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            },
            // 新規作成
            None => {
                sqlx::query(
                    "INSERT INTO products (name, price, description, quantity, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
                )
                .bind(&product.name)
                .bind(product.price)
                .bind(&product.description)
                .bind(product.quantity)
                .bind(&now)
                .bind(&now)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            }
        }
        
        Ok(())
    }
}
