use std::sync::Arc;

use crate::application::commands::CalculateCartHandler;
use crate::application::commands::handlers::CreateOrderHandler;
use crate::application::queries::handlers::{
    GetPaymentMethodListHandler, GetShippingMethodListHandler,
};
use crate::application::repositories::{
    CategoryRepository, ColorRepository, OrderRepository, PaymentMethodRepository,
    ProductRepository, ShippingMethodRepository, VariantRepository,
};
use crate::application::{
    Dispatcher, FindVariantsHandler, GetCategoryListHandler, GetColorListHandler,
    GetProductHandler, GetProductListHandler,
};
use crate::infrastructure::database::db::get_db;
use crate::infrastructure::database::repositories_impl::{
    SqliteCategoryRepository, SqliteColorRepository, SqliteOrderRepository,
    SqlitePaymentMethodRepository, SqliteProductRepository, SqliteShippingMethodRepository,
    SqliteVariantRepository,
};

/// コンテナはアプリケーションの依存関係を管理します
/// Uncle Bob's Clean Architecture: Frameworks & Drivers層でDI設定
pub struct Container {
    /// ProductRepositoryの実装
    pub product_repository: Arc<dyn ProductRepository + Send + Sync>,
    /// CategoryRepositoryの実装
    pub category_repository: Arc<dyn CategoryRepository + Send + Sync>,
    /// ColorRepositoryの実装
    pub color_repository: Arc<dyn ColorRepository + Send + Sync>,
    /// VariantRepositoryの実装
    pub variant_repository: Arc<dyn VariantRepository + Send + Sync>,
    /// ShippingMethodRepositoryの実装
    pub shipping_method_repository: Arc<dyn ShippingMethodRepository + Send + Sync>,
    /// PaymentMethodRepositoryの実装
    pub payment_method_repository: Arc<dyn PaymentMethodRepository + Send + Sync>,
    /// OrderRepositoryの実装
    pub order_repository: Arc<dyn OrderRepository + Send + Sync>,
    /// CQRSディスパッチャ
    pub dispatcher: Arc<Dispatcher>,
}

impl Container {
    /// 新しいコンテナを作成します（本番環境用）
    pub async fn new() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // データベースプールを取得
        let db = get_db().await?;
        let pool = db.get_pool().clone();

        Self::new_with_pool(pool).await
    }

    /// テスト用コンテナを作成します
    pub async fn new_for_test() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // テスト用インメモリDB
        let pool = sqlx::SqlitePool::connect("sqlite::memory:").await?;
        Self::new_with_pool(pool).await
    }

    /// プールを指定してコンテナを作成します
    async fn new_with_pool(
        pool: sqlx::SqlitePool,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // リポジトリの実装をインスタンス化（プールを注入）
        let product_repository = Arc::new(SqliteProductRepository::new(pool.clone()));
        let category_repository = Arc::new(SqliteCategoryRepository::new(pool.clone()));
        let color_repository = Arc::new(SqliteColorRepository::new(pool.clone()));
        let variant_repository = Arc::new(SqliteVariantRepository::new(pool.clone()));
        let shipping_method_repository =
            Arc::new(SqliteShippingMethodRepository::new(Arc::new(pool.clone())));
        let payment_method_repository = Arc::new(SqlitePaymentMethodRepository::new(pool.clone()));
        let order_repository = Arc::new(SqliteOrderRepository::new(pool.clone()));

        // ハンドラを作成
        let calculate_cart_handler = Arc::new(CalculateCartHandler::new(
            product_repository.clone(),
            shipping_method_repository.clone(),
            payment_method_repository.clone(),
        ));
        let get_product_handler = Arc::new(GetProductHandler::new(product_repository.clone()));
        let get_product_list_handler =
            Arc::new(GetProductListHandler::new(product_repository.clone()));
        let get_category_list_handler =
            Arc::new(GetCategoryListHandler::new(category_repository.clone()));
        let get_color_list_handler = Arc::new(GetColorListHandler::new(color_repository.clone()));
        let find_variants_handler = Arc::new(FindVariantsHandler::new(variant_repository.clone()));
        let get_shipping_method_list_handler = Arc::new(GetShippingMethodListHandler::new(
            shipping_method_repository.clone(),
        ));
        let get_payment_method_list_handler = Arc::new(GetPaymentMethodListHandler::new(
            payment_method_repository.clone(),
        ));
        let create_order_handler = Arc::new(CreateOrderHandler::new(
            product_repository.clone(),
            shipping_method_repository.clone(),
            payment_method_repository.clone(),
            order_repository.clone(),
        ));

        // ディスパッチャを作成
        let dispatcher = Arc::new(Dispatcher::new(
            calculate_cart_handler,
            create_order_handler,
            get_product_handler,
            get_product_list_handler,
            get_category_list_handler,
            get_color_list_handler,
            find_variants_handler,
            get_shipping_method_list_handler,
            get_payment_method_list_handler,
        ));

        Ok(Self {
            product_repository,
            category_repository,
            color_repository,
            variant_repository,
            shipping_method_repository,
            payment_method_repository,
            order_repository,
            dispatcher,
        })
    }

    /// Dispatcherを取得します
    pub fn get_dispatcher(&self) -> Arc<Dispatcher> {
        self.dispatcher.clone()
    }
}

/// グローバルなコンテナインスタンスを取得します（本番環境用）
pub async fn get_container() -> Result<Container, Box<dyn std::error::Error + Send + Sync>> {
    Container::new().await
}

/// テスト用コンテナインスタンスを取得します
pub async fn get_test_container() -> Result<Container, Box<dyn std::error::Error + Send + Sync>> {
    Container::new_for_test().await
}
