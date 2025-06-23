use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::error::RepositoryError;
use crate::application::repositories::CategoryRepository;
use crate::application::dto::{CategoryListDTO, CategoryDTO};
use crate::infrastructure::database::db::get_db;

/// SQLite実装のCategoryRepository
/// Clean Architecture: Frameworks & Drivers層
/// CQRS Query側専用：CategoryDTOを直接構築してパフォーマンス重視
pub struct SqliteCategoryRepository {
    pool: SqlitePool,
}

impl SqliteCategoryRepository {
    pub fn new() -> Self {
        // データベースプールは実際のクエリ実行時に取得する
        // 初期化時に非同期で取得することはできないため、プレースホルダーとして空のプールを使用
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        Self { pool }
    }

    /// データベースプールを取得（実際のクエリ実行時に使用）
    async fn get_pool(&self) -> Result<SqlitePool, RepositoryError> {
        get_db().await
            .map(|db| db.get_pool().clone())
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))
    }
}

#[async_trait]
impl CategoryRepository for SqliteCategoryRepository {
    async fn find_all(&self) -> Result<CategoryListDTO, RepositoryError> {
        let pool = self.get_pool().await?;

        // カテゴリ一覧を取得（display_orderでソート）
        let category_rows = sqlx::query(
            r#"
            SELECT 
                id,
                name,
                slug,
                parent_id,
                display_order
            FROM categories
            ORDER BY display_order ASC, name ASC
            "#
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // CategoryDTOのリストを構築
        let mut categories = Vec::new();

        for row in category_rows {
            let id: String = row.try_get("id")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let name: String = row.try_get("name")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let slug: String = row.try_get("slug")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let parent_id: Option<String> = row.try_get("parent_id")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let display_order: i64 = row.try_get("display_order")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

            // CategoryDTOを構築
            let category_dto = CategoryDTO::new(
                id,
                name,
                slug,
                parent_id,
                display_order as u32,
            );

            categories.push(category_dto);
        }

        // CategoryListDTOを構築して返す
        Ok(CategoryListDTO::new(categories))
    }
}
