use axum::extract::State;
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::frameworks_and_drivers::Container;
use crate::error::{Error, Result};
use crate::interface_adapters::products::presenters::ProductPresenter;

/// Get All Products Controller - 商品一覧取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetProductsController;

impl GetProductsController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/products", get(Self::handle))
    }

    /// GET /products - 商品一覧取得処理
    async fn handle(
        State(container): State<Arc<Container>>
    ) -> Result<Json<Vec<ProductPresenter>>> {
        let get_all_products_usecase = container.create_get_all_products_usecase();
        
        let products = get_all_products_usecase
            .get_all()
            .await
            .map_err(|_| Error::InternalServerError)?;
            
        let presenters: Vec<ProductPresenter> = products.into_iter().map(|p| p.into()).collect();
        Ok(Json(presenters))
    }
} 