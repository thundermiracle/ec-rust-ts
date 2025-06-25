use std::sync::Arc;

use crate::infrastructure::database::repositories_impl::{SqliteProductRepository, SqliteCategoryRepository, SqliteColorRepository};
use crate::application::repositories::{ProductRepository, CategoryRepository};
use crate::application::{
    Dispatcher,
    BuyProductHandler,
    GetProductHandler, 
    GetProductListHandler,
    GetCategoryListHandler,
    GetColorListHandler,
};

/// コンテナはアプリケーションの依存関係を管理します
/// Uncle Bob's Clean Architecture: Frameworks & Drivers層でDI設定
pub struct Container {
    /// ProductRepositoryの実装
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
    /// CategoryRepositoryの実装
    pub category_repository: Arc<dyn CategoryRepository + Send + Sync>,
    /// CQRSディスパッチャ
    pub dispatcher: Arc<Dispatcher>,
}

impl Container {
    /// 新しいコンテナを作成します
    pub fn new() -> Self {
        // リポジトリの実装をインスタンス化
        let product_repository = Arc::new(SqliteProductRepository::new());
        let category_repository = Arc::new(SqliteCategoryRepository::new());
        let color_repository = Arc::new(SqliteColorRepository::new());
        
        // ハンドラを作成
        let buy_product_handler = Arc::new(BuyProductHandler::new(product_repository.clone()));
        let get_product_handler = Arc::new(GetProductHandler::new(product_repository.clone()));
        let get_product_list_handler = Arc::new(GetProductListHandler::new(product_repository.clone()));
        let get_category_list_handler = Arc::new(GetCategoryListHandler::new(category_repository.clone()));
        let get_color_list_handler = Arc::new(GetColorListHandler::new(color_repository.clone()));
        
        // ディスパッチャを作成
        let dispatcher = Arc::new(Dispatcher::new(
            buy_product_handler,
            get_product_handler, 
            get_product_list_handler,
            get_category_list_handler,
            get_color_list_handler,
        ));
        
        Self {
            product_repository,
            category_repository,
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