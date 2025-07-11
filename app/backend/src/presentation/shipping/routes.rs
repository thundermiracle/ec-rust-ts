use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::shipping::controllers::GetShippingMethodListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetShippingMethodListController::routes())
}