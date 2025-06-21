pub mod controllers;
pub mod presenters;
pub mod requests;
pub mod responses;

use axum::Router;
use std::sync::Arc;
use crate::frameworks_and_drivers::Container;

pub use controllers::{GetProductController, CategoriesController, ColorsController, GetProductListController};
pub use requests::BuyProductRequest;
pub use presenters::{ProductPresenter, ProductListPresenter};
pub use responses::{ProductResponse, ProductListResponse};

/// Products モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(GetProductController::routes())
        .merge(GetProductListController::routes())
        .merge(CategoriesController::routes())
        .merge(ColorsController::routes())
} 