use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::dto::ProductListDTO;

/// 商品リスト取得クエリハンドラ
/// CQRS パターンに基づく読み取り操作のハンドラ
pub struct GetProductListHandler {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductListHandler {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    /// 商品リスト取得クエリを実行
    /// 
    /// # Returns
    /// * `Result<ProductListDTO, ApplicationError>` - 成功時は商品リストデータ、失敗時はエラー
    pub async fn handle(&self) -> Result<ProductListDTO, ApplicationError> {
        println!("->> get_product_list_handler");
        
        let product_list = self.product_repository.find_all().await?;

        Ok(product_list)
    }
} 