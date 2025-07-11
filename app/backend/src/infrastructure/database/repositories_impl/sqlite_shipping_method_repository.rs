use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

use crate::application::repositories::ShippingMethodRepository;
use crate::application::error::RepositoryError;
use crate::application::dto::{ShippingMethodListDTO, ShippingMethodDTO};

pub struct SqliteShippingMethodRepository {
    pool: Arc<SqlitePool>,
}

impl SqliteShippingMethodRepository {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ShippingMethodRepository for SqliteShippingMethodRepository {
    async fn find_all(&self) -> Result<ShippingMethodListDTO, RepositoryError> {
        let rows = sqlx::query(
            r#"
            SELECT id, name, description, price, is_active, sort_order, created_at, updated_at
            FROM shipping_methods
            WHERE is_active = 1
            ORDER BY sort_order ASC
            "#
        )
        .fetch_all(self.pool.as_ref())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut methods = Vec::new();
        for row in rows {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            let price: i64 = row.get("price");

            let dto = ShippingMethodDTO {
                id,
                name,
                description,
                price: price as u32,
            };
            methods.push(dto);
        }

        Ok(ShippingMethodListDTO::new(methods))
    }
}

