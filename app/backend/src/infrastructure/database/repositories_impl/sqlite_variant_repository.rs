use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use crate::application::repositories::VariantRepository;
use crate::application::dto::VariantSummaryDTO;
use crate::application::error::{ApplicationError, RepositoryError};
use crate::domain::models::SKUId;

pub struct SqliteVariantRepository {
    pool: SqlitePool,
}

impl SqliteVariantRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl VariantRepository for SqliteVariantRepository {
    async fn find_by_ids(&self, ids: Vec<SKUId>) -> Result<Vec<VariantSummaryDTO>, ApplicationError> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        // Convert SKUIds to UUIDs
        let uuid_strings: Vec<String> = ids.iter().map(|id| id.value().to_string()).collect();
        
        // Create placeholders for the IN clause
        let placeholders = format!("?{}", ", ?".repeat(uuid_strings.len() - 1));
        
        let query = format!(
            r#"
            SELECT 
                id,
                base_price,
                sale_price,
                image_url,
                material,
                dimensions
            FROM skus 
            WHERE id IN ({})
            "#,
            placeholders
        );

        let mut query_builder = sqlx::query(&query);
        
        // Bind all the UUID strings
        for uuid_string in &uuid_strings {
            query_builder = query_builder.bind(uuid_string);
        }

        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ApplicationError::Repository(RepositoryError::DatabaseError(e.to_string())))?;

        let mut variants = Vec::new();
        for row in rows {
            let id_str: String = row.get("id");
            let uuid = Uuid::parse_str(&id_str)
                .map_err(|e| ApplicationError::Repository(RepositoryError::DatabaseError(format!("Invalid UUID: {}", e))))?;
            
            let variant = VariantSummaryDTO::new(
                SKUId::from_uuid(uuid),
                row.get("base_price"),
                row.get("sale_price"),
                row.get("image_url"),
                row.get("material"),
                row.get("dimensions"),
            );
            variants.push(variant);
        }

        Ok(variants)
    }
} 