pub mod controllers;
pub mod presenters;
pub mod requests;
pub mod responses;

use axum::Router;
use std::sync::Arc;
use crate::frameworks_and_drivers::Container;

pub use controllers::{GetProductController, CategoriesController, ColorsController};
pub use requests::BuyProductRequest;
pub use presenters::ProductPresenter;
pub use responses::ProductResponse;

/// Products モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetProductController::routes())
        .merge(CategoriesController::routes())
        .merge(ColorsController::routes())
} 