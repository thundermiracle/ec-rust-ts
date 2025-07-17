use crate::infrastructure::Container;
use crate::presentation::cart::controllers::CalculateCartController;
use axum::Router;
use std::sync::Arc;

/// カート関連のルーティング
pub fn routes() -> Router<Arc<Container>> {
    Router::new().merge(CalculateCartController::routes())
}
