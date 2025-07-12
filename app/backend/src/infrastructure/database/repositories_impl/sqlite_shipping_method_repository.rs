use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use std::sync::Arc;

use crate::application::repositories::ShippingMethodRepository;
use crate::application::error::RepositoryError;
use crate::application::dto::{ShippingMethodListDTO, ShippingMethodDTO};
use crate::domain::entities::ShippingMethod;
use crate::domain::value_objects::{ShippingMethodId, Money};

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

    async fn find_by_id_dto(&self, id: &str) -> Result<Option<ShippingMethodDTO>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, price, is_active
            FROM shipping_methods
            WHERE id = ? AND is_active = 1
            "#
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                let dto = ShippingMethodDTO {
                    id: row.get("id"),
                    name: row.get("name"),
                    description: row.get("description"),
                    price: row.get::<i64, _>("price") as u32,
                };
                Ok(Some(dto))
            }
            None => Ok(None),
        }
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<ShippingMethod>, RepositoryError> {
        let row = sqlx::query(
            r#"
            SELECT id, name, description, price, is_active, sort_order, created_at, updated_at
            FROM shipping_methods
            WHERE id = ?
            "#
        )
        .bind(id)
        .fetch_optional(self.pool.as_ref())
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        match row {
            Some(row) => {
                use chrono::{DateTime, Utc};
                
                let shipping_method_id = ShippingMethodId::new(row.get::<String, _>("id"))
                    .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;
                
                let shipping_method = ShippingMethod::with_timestamps(
                    shipping_method_id,
                    row.get::<String, _>("name"),
                    row.get::<String, _>("description"),
                    Money::from_yen(row.get::<i64, _>("price") as u32),
                    row.get::<bool, _>("is_active"),
                    row.get::<u32, _>("sort_order"),
                    row.get::<DateTime<Utc>, _>("created_at"),
                    row.get::<DateTime<Utc>, _>("updated_at"),
                );
                
                Ok(Some(shipping_method))
            }
            None => Ok(None),
        }
    }
}

