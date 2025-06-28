use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::colors::controllers::GetColorListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetColorListController::routes())
} 