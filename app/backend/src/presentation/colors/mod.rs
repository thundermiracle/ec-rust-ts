pub mod controllers;
mod responses;
mod presenters;

pub use responses::{ColorListResponse, ColorListItemResponse};

use std::sync::Arc;
use axum::Router;
use crate::infrastructure::Container;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(controllers::GetColorListController::routes())
}
