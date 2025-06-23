pub mod controllers;
pub mod presenters;
pub mod responses;

use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;

pub use controllers::GetCategoryListController;
pub use presenters::CategoryListPresenter;
pub use responses::{CategoryListResponse, CategoryResponse};

/// Categories モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetCategoryListController::routes())
}
