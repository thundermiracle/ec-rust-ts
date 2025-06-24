use axum::extract::State;
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::error::Result;
use crate::presentation::categories::{CategoryListPresenter, CategoryListResponse};
use crate::presentation::ErrorResponse;

/// Get Category List Controller - カテゴリリスト取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetCategoryListController;

impl GetCategoryListController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/categories", get(handle))
    }
}

/// GET /categories - カテゴリリスト取得処理
/// 統合されたカテゴリリスト情報を返す
#[utoipa::path(
    get,
    path = "/categories",
    operation_id = "get_category_list",
    responses(
        (status = 200, description = "カテゴリリスト取得成功", body = CategoryListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Categories"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<CategoryListResponse>> {
    println!("->> GetCategoryListController::handle");
    
    let dispatcher = container.get_dispatcher();
    
    let category_list = dispatcher
        .execute_get_category_list_query()
        .await?; // ApplicationErrorからErrorへの自動変換を利用
        
    println!("->> GetCategoryListController::handle - success for category list");
    Ok(Json(CategoryListPresenter::present(category_list)))
}
