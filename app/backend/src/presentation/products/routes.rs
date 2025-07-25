use super::controllers;
use crate::infrastructure::Container;
use axum::Router;
use std::sync::Arc;

/// Products モジュールの全ルート定義
/// Clean Architecture: 関連するControllerのルートを統合
pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(controllers::GetProductController::routes())
        .merge(controllers::GetProductListController::routes())
}
