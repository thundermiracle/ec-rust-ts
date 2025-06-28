use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::categories::controllers::GetCategoryListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetCategoryListController::routes())
} 