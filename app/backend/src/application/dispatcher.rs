use std::sync::Arc;

use crate::application::commands::handlers::CalculateCartHandler;
use crate::application::queries::handlers::{GetProductHandler, GetProductListHandler, GetCategoryListHandler, GetColorListHandler, FindVariantsHandler, GetShippingMethodListHandler, GetPaymentMethodListHandler};
use crate::application::commands::models::CalculateCartCommand;
use crate::application::queries::models::{GetProductQuery, FindVariantsQuery};
use crate::application::error::ApplicationError;
use crate::application::dto::{CategoryListDTO, ColorListDTO, ProductDTO, ProductListDTO, VariantSummaryDTO, ShippingMethodListDTO, PaymentMethodListDTO, CalculateCartResultDto};

/// CQRS パターンのコマンド・クエリディスパッチャ
/// 
/// コマンドとクエリの実行を一元的に管理し、適切なハンドラに処理を委譲します。
/// これにより、プレゼンテーション層はビジネスロジックの詳細を知る必要がなくなります。
pub struct Dispatcher {
    // コマンドハンドラ
    calculate_cart_handler: Arc<CalculateCartHandler>,
    
    // クエリハンドラ
    get_product_handler: Arc<GetProductHandler>,
    get_product_list_handler: Arc<GetProductListHandler>,
    get_category_list_handler: Arc<GetCategoryListHandler>,
    get_color_list_handler: Arc<GetColorListHandler>,
    find_variants_handler: Arc<FindVariantsHandler>,
    get_shipping_method_list_handler: Arc<GetShippingMethodListHandler>,
    get_payment_method_list_handler: Arc<GetPaymentMethodListHandler>,
}

impl Dispatcher {
    pub fn new(
        calculate_cart_handler: Arc<CalculateCartHandler>,
        get_product_handler: Arc<GetProductHandler>,
        get_product_list_handler: Arc<GetProductListHandler>,
        get_category_list_handler: Arc<GetCategoryListHandler>,
        get_color_list_handler: Arc<GetColorListHandler>,
        find_variants_handler: Arc<FindVariantsHandler>,
        get_shipping_method_list_handler: Arc<GetShippingMethodListHandler>,
        get_payment_method_list_handler: Arc<GetPaymentMethodListHandler>,
    ) -> Self {
        Self {
            calculate_cart_handler,
            get_product_handler,
            get_product_list_handler,
            get_category_list_handler,
            get_color_list_handler,
            find_variants_handler,
            get_shipping_method_list_handler,
            get_payment_method_list_handler,
        }
    }

    /// カート計算コマンドを実行
    pub async fn execute_calculate_cart_command(&self, command: CalculateCartCommand) -> Result<CalculateCartResultDto, ApplicationError> {
        self.calculate_cart_handler.handle(command).await
    }

    /// 商品取得クエリを実行
    pub async fn execute_get_product_query(&self, query: GetProductQuery) -> Result<ProductDTO, ApplicationError> {
        self.get_product_handler.handle(query).await
    }

    /// 商品リスト取得クエリを実行
    pub async fn execute_get_product_list_query(&self) -> Result<ProductListDTO, ApplicationError> {
        self.get_product_list_handler.handle().await
    }

    /// カテゴリリスト取得クエリを実行
    pub async fn execute_get_category_list_query(&self) -> Result<CategoryListDTO, ApplicationError> {
        self.get_category_list_handler.handle().await
    }

    /// 色リスト取得クエリを実行
    pub async fn execute_get_color_list_query(&self) -> Result<ColorListDTO, ApplicationError> {
        self.get_color_list_handler.handle().await
    }

    /// バリアントリスト取得クエリを実行
    pub async fn execute_find_variants_query(&self, query: FindVariantsQuery) -> Result<Vec<VariantSummaryDTO>, ApplicationError> {
        self.find_variants_handler.handle(query).await
    }

    /// 配送方法リスト取得クエリを実行
    pub async fn execute_get_shipping_method_list_query(&self) -> Result<ShippingMethodListDTO, ApplicationError> {
        self.get_shipping_method_list_handler.handle().await
    }

    /// 支払い方法リスト取得クエリを実行
    pub async fn execute_get_payment_method_list_query(&self) -> Result<PaymentMethodListDTO, ApplicationError> {
        self.get_payment_method_list_handler.handle().await
    }
} 