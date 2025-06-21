use std::sync::Arc;

use crate::frameworks_and_drivers::persistence::repositories_impl::SqliteProductRepository;
use crate::application::repositories::ProductRepository;
use crate::application::use_cases::{GetProductListUseCase, GetProductUseCase};

/// コンテナはアプリケーションの依存関係を管理します
/// Uncle Bob's Clean Architecture: Frameworks & Drivers層でDI設定
pub struct Container {
    /// ProductRepositoryの実装
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl Container {
    /// 新しいコンテナを作成します
    pub fn new() -> Self {
        // リポジトリの実装をインスタンス化
        let product_repository = Arc::new(SqliteProductRepository::new());
        
        Self {
            product_repository,
        }
    }
    
    /// GetProductUseCaseを作成します
    pub fn create_get_product_usecase(&self) -> GetProductUseCase {
        GetProductUseCase::new(self.product_repository.clone())
    }    

    /// GetProductListUseCaseを作成します
    pub fn create_get_product_list_usecase(&self) -> GetProductListUseCase {
        GetProductListUseCase::new(self.product_repository.clone())
    }
}

/// グローバルなコンテナインスタンスを取得します
pub fn get_container() -> Container {
    Container::new()
}