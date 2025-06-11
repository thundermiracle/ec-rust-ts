use std::sync::Arc;

use crate::frameworks_and_drivers::persistence::repositories_impl::SqliteProductRepository;
use crate::application::repositories::ProductRepository;
use crate::application::use_cases::{GetProductUseCase, GetAllProductsUseCase, BuyProductUseCase};

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
    
    /// GetAllProductsUseCaseを作成します
    pub fn create_get_all_products_usecase(&self) -> GetAllProductsUseCase {
        GetAllProductsUseCase::new(self.product_repository.clone())
    }
    
    /// BuyProductUseCaseを作成します
    pub fn create_buy_product_usecase(&self) -> BuyProductUseCase {
        BuyProductUseCase::new(self.product_repository.clone())
    }
}

/// グローバルなコンテナインスタンスを取得します
pub fn get_container() -> Container {
    Container::new()
}