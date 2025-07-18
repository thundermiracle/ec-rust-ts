use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::orders::CreateOrderController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new().merge(CreateOrderController::routes())
}