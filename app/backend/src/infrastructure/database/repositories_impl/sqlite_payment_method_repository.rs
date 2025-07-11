use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::repositories::PaymentMethodRepository;
use crate::application::error::RepositoryError;
use crate::application::dto::{PaymentMethodListDTO, PaymentMethodDTO};

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
}