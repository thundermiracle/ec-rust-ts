use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::repositories::PaymentMethodRepository;
use crate::application::error::RepositoryError;
use crate::application::dto::{PaymentMethodListDTO, PaymentMethodDTO};
use crate::domain::entities::PaymentMethod;

/// SQLite実装のPaymentMethodRepository
/// Clean Architecture: Infrastructure層
pub struct SqlitePaymentMethodRepository {
    pool: SqlitePool,
}

impl SqlitePaymentMethodRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PaymentMethodRepository for SqlitePaymentMethodRepository {
    async fn find_all(&self) -> Result<PaymentMethodListDTO, RepositoryError> {
        let rows = sqlx::query("SELECT id, name, description FROM payment_methods WHERE is_active = 1 ORDER BY sort_order")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let items: Vec<PaymentMethodDTO> = rows
            .into_iter()
            .map(|row| PaymentMethodDTO {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .collect();

        Ok(PaymentMethodListDTO::new(items))
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<PaymentMethod>, RepositoryError> {
        let row = sqlx::query("SELECT id, name, description, is_active, sort_order FROM payment_methods WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        match row {
            Some(row) => {
                let payment_method = PaymentMethod::new(
                    row.get::<String, _>("id"),
                    row.get::<String, _>("name"),
                    row.get::<String, _>("description"),
                    row.get::<bool, _>("is_active"),
                    row.get::<u32, _>("sort_order"),
                ).map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
                
                Ok(Some(payment_method))
            }
            None => Ok(None),
        }
    }
}