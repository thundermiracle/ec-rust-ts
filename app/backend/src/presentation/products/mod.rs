pub mod controllers;
pub mod presenters;
pub mod requests;
pub mod responses;

use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;

pub use controllers::{GetProductController, GetProductListController};
pub use presenters::ProductListPresenter;
pub use responses::ProductListResponse;

/// Products モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetProductController::routes())
        .merge(GetProductListController::routes())
}
