use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::variants::controllers::FindVariantsController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(FindVariantsController::routes())
} 