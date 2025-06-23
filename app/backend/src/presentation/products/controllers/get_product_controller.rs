use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::error::Result;
use crate::application::queries::GetProductQuery;
use crate::presentation::products::presenters::ProductPresenter;
use crate::presentation::products::responses::ProductResponse;

/// Get Product Controller - 商品詳細取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetProductController;

impl GetProductController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/products/{id}", get(Self::handle))
    }

    /// GET /products/{id} - 商品詳細取得処理
    /// 統合されたリッチな商品情報を返す
    async fn handle(
        State(container): State<Arc<Container>>,
        Path(id): Path<String>
    ) -> Result<Json<ProductResponse>> {
        println!("->> GetProductController::handle - product_id: {}", id);
        
        let get_product_usecase = container.create_get_product_usecase();
        
        let product_detail = get_product_usecase
            .get_by_id(GetProductQuery::new(id.clone()))
            .await?; // ApplicationErrorからErrorへの自動変換を利用
            
        println!("->> GetProductController::handle - success for product_id: {}", id);
        Ok(Json(ProductPresenter::present(product_detail)))
    }
} 