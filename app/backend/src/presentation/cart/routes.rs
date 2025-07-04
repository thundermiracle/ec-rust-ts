use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;
use crate::presentation::cart::controllers::CalculateCartController;

/// カート関連のルーティング
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(CalculateCartController::routes())
}
