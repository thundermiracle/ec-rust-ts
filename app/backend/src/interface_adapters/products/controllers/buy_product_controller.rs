use axum::extract::{Path, State};
use axum::{routing::post, Json, Router};
use std::sync::Arc;

use crate::frameworks_and_drivers::Container;
use crate::error::{Error, Result};
use crate::application::commands::BuyProductCommand;
use crate::application::ApplicationError;
use crate::interface_adapters::products::requests::BuyProductRequest;

/// Buy Product Controller - 商品購入の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct BuyProductController;

impl BuyProductController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/products/{id}/buy", post(Self::handle))
    }

    /// POST /products/{id}/buy - 商品購入処理
    async fn handle(
        State(container): State<Arc<Container>>,
        Path(id): Path<u32>, 
        Json(request): Json<BuyProductRequest>
    ) -> Result<()> {
        let buy_product_usecase = container.create_buy_product_usecase();
        
        // RequestからCommandへの変換
        let command = BuyProductCommand {
            quantity: request.quantity,
        };
        
        buy_product_usecase
            .buy(id, command)
            .await
            .map_err(|e| match e {
                ApplicationError::ProductNotFound(_) => Error::NotFound,
                ApplicationError::Domain(_) => Error::BuyProductFailed,
                _ => Error::InternalServerError,
            })
    }
} 