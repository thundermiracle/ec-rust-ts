use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::frameworks_and_drivers::Container;
use crate::error::{Error, Result};
use crate::application::ApplicationError;
use crate::interface_adapters::products::presenters::ProductPresenter;

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
    async fn handle(
        State(container): State<Arc<Container>>,
        Path(id): Path<u32>
    ) -> Result<Json<ProductPresenter>> {
        let get_product_usecase = container.create_get_product_usecase();
        
        let product = get_product_usecase
            .get_by_id(id)
            .await
            .map_err(|e| match e {
                ApplicationError::ProductNotFound(_) => Error::NotFound,
                _ => Error::InternalServerError,
            })?;
            
        Ok(Json(product.into()))
    }
} 