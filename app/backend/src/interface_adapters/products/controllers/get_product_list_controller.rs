use axum::extract::State;
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::frameworks_and_drivers::Container;
use crate::error::Result;
use crate::interface_adapters::products::{ProductListPresenter, ProductListResponse};

/// Get Product Controller - 商品詳細取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetProductListController;

impl GetProductListController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/products", get(Self::handle))
    }

    /// GET /products/{id} - 商品詳細取得処理
    /// 統合されたリッチな商品情報を返す
    async fn handle(
        State(container): State<Arc<Container>>,
    ) -> Result<Json<ProductListResponse>> {
        println!("->> GetProductListController::handle");
        
        let get_product_list_usecase = container.create_get_product_list_usecase();
        
        let product_list = get_product_list_usecase
            .get_all()
            .await?; // ApplicationErrorからErrorへの自動変換を利用
            
        println!("->> GetProductListController::handle - success for product_list");
        Ok(Json(ProductListPresenter::present(product_list)))
    }
} 