use std::sync::Arc;

use crate::infrastructure::database::repositories_impl::SqliteProductRepository;
use crate::application::repositories::ProductRepository;
use crate::application::{
    Dispatcher,
    BuyProductHandler,
    GetProductHandler, 
    GetProductListHandler
};

/// コンテナはアプリケーションの依存関係を管理します
/// Uncle Bob's Clean Architecture: Frameworks & Drivers層でDI設定
pub struct Container {
    /// ProductRepositoryの実装
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
    /// CQRSディスパッチャ
    pub dispatcher: Arc<Dispatcher>,
}

impl Container {
    /// 新しいコンテナを作成します
    pub fn new() -> Self {
        // リポジトリの実装をインスタンス化
        let product_repository = Arc::new(SqliteProductRepository::new());
        
        // ハンドラを作成
        let buy_product_handler = Arc::new(BuyProductHandler::new(product_repository.clone()));
        let get_product_handler = Arc::new(GetProductHandler::new(product_repository.clone()));
        let get_product_list_handler = Arc::new(GetProductListHandler::new(product_repository.clone()));
        
        // ディスパッチャを作成
        let dispatcher = Arc::new(Dispatcher::new(
            buy_product_handler,
            get_product_handler, 
            get_product_list_handler,
        ));
        
        Self {
            product_repository,
            dispatcher,
        }
    }
    
    /// Dispatcherを取得します
    pub fn get_dispatcher(&self) -> Arc<Dispatcher> {
        self.dispatcher.clone()
    }
}

/// グローバルなコンテナインスタンスを取得します
pub fn get_container() -> Container {
    Container::new()
}