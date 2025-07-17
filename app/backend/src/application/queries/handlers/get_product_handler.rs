use std::sync::Arc;

use crate::application::dto::ProductDTO;
use crate::application::error::ApplicationError;
use crate::application::queries::models::GetProductQuery;
use crate::application::repositories::ProductRepository;

/// 商品取得クエリハンドラ
/// CQRS パターンに基づく読み取り操作のハンドラ
pub struct GetProductHandler {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductHandler {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self { product_repository }
    }

    /// 商品取得クエリを実行
    ///
    /// # Arguments
    /// * `query` - 取得クエリデータ
    ///
    /// # Returns
    /// * `Result<ProductDTO, ApplicationError>` - 成功時は商品データ、失敗時はエラー
    pub async fn handle(&self, query: GetProductQuery) -> Result<ProductDTO, ApplicationError> {
        println!("->> get_product_handler: product_id={:?}", query.product_id);

        let product = self
            .product_repository
            .find_by_id(&query.product_id)
            .await?
            .ok_or(ApplicationError::ProductNotFound(
                query.product_id.to_string(),
            ))?;

        Ok(product)
    }
}
