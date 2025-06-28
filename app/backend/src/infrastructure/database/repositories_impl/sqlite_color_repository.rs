use async_trait::async_trait;
use sqlx::{SqlitePool, Row};

use crate::application::dto::ColorDTO;
use crate::application::{dto::ColorListDTO, error::RepositoryError};
use crate::application::repositories::ColorRepository;

pub struct SqliteColorRepository {
    pool: SqlitePool,
}

impl SqliteColorRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ColorRepository for SqliteColorRepository {
    async fn find_all(&self) -> Result<ColorListDTO, RepositoryError> {
        let color_rows = sqlx::query(
            r#"
            SELECT id, name, hex FROM colors
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let colors = color_rows.into_iter().map(|row| ColorDTO {
            id: row.get("id"),
            name: row.get("name"),
            hex: row.get("hex"),
        }).collect();

        Ok(ColorListDTO::new(colors))
    }
}