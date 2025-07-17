use axum::extract::State;
use axum::{Json, Router, routing::get};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::ErrorResponse;
use crate::presentation::products::presenters::GetProductListPresenter;
use crate::presentation::products::responses::GetProductListResponse;

/// Get Product List Controller - 商品リスト取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetProductListController;

impl GetProductListController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/products", get(handle))
    }
}

/// GET /products - 商品リスト取得処理
/// 統合されたリッチな商品リスト情報を返す
#[utoipa::path(
    get,
    path = "/products",
    operation_id = "get_product_list",
    responses(
        (status = 200, description = "商品リスト取得成功", body = GetProductListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<GetProductListResponse>> {
    println!("->> GetProductListController::handle");

    let dispatcher = container.get_dispatcher();

    let product_list = dispatcher.execute_get_product_list_query().await?; // ApplicationErrorからErrorへの自動変換を利用

    println!("->> GetProductListController::handle - success for product_list");
    Ok(Json(GetProductListPresenter::present(product_list)))
}
