use std::sync::Arc;

use crate::application::dto::CategoryListDTO;
use crate::application::error::ApplicationError;
use crate::application::repositories::CategoryRepository;

/// カテゴリリスト取得クエリハンドラ
/// CQRS パターンに基づく読み取り操作のハンドラ
pub struct GetCategoryListHandler {
    category_repository: Arc<dyn CategoryRepository + Send + Sync>,
}

impl GetCategoryListHandler {
    pub fn new(category_repository: Arc<dyn CategoryRepository + Send + Sync>) -> Self {
        Self {
            category_repository,
        }
    }

    /// カテゴリリスト取得クエリを実行
    ///
    /// # Returns
    /// * `Result<CategoryListDTO, ApplicationError>` - 成功時はカテゴリリストデータ、失敗時はエラー
    pub async fn handle(&self) -> Result<CategoryListDTO, ApplicationError> {
        println!("->> get_category_list_handler");

        let category_list = self.category_repository.find_all().await?;

        Ok(category_list)
    }
}
