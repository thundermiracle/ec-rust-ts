use std::sync::Arc;

use crate::application::commands::handlers::BuyProductHandler;
use crate::application::queries::handlers::{GetProductHandler, GetProductListHandler};
use crate::application::commands::models::BuyProductCommand;
use crate::application::queries::models::GetProductQuery;
use crate::application::error::ApplicationError;
use crate::application::dto::{ProductDTO, ProductListDTO};

/// CQRS パターンのコマンド・クエリディスパッチャ
/// 
/// コマンドとクエリの実行を一元的に管理し、適切なハンドラに処理を委譲します。
/// これにより、プレゼンテーション層はビジネスロジックの詳細を知る必要がなくなります。
pub struct Dispatcher {
    // コマンドハンドラ
    buy_product_handler: Arc<BuyProductHandler>,
    
    // クエリハンドラ
    get_product_handler: Arc<GetProductHandler>,
    get_product_list_handler: Arc<GetProductListHandler>,
}

impl Dispatcher {
    pub fn new(
        buy_product_handler: Arc<BuyProductHandler>,
        get_product_handler: Arc<GetProductHandler>,
        get_product_list_handler: Arc<GetProductListHandler>,
    ) -> Self {
        Self {
            buy_product_handler,
            get_product_handler,
            get_product_list_handler,
        }
    }

    /// 商品購入コマンドを実行
    pub async fn execute_buy_product_command(&self, command: BuyProductCommand) -> Result<(), ApplicationError> {
        self.buy_product_handler.handle(command).await
    }

    /// 商品取得クエリを実行
    pub async fn execute_get_product_query(&self, query: GetProductQuery) -> Result<ProductDTO, ApplicationError> {
        self.get_product_handler.handle(query).await
    }

    /// 商品リスト取得クエリを実行
    pub async fn execute_get_product_list_query(&self) -> Result<ProductListDTO, ApplicationError> {
        self.get_product_list_handler.handle().await
    }
} 