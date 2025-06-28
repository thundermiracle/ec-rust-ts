use axum::extract::{Path, State};
use axum::{routing::get, Json, Router};
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::error::Result;
use crate::application::GetProductQuery;
use crate::presentation::products::presenters::GetProductPresenter;
use crate::presentation::products::responses::GetProductResponse;
use crate::presentation::ErrorResponse;

/// Get Product Controller - 商品詳細取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct GetProductController;

impl GetProductController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/products/{id}", get(handle))
    }
}

/// GET /products/{id} - 商品詳細取得処理
/// 統合されたリッチな商品情報を返す
#[utoipa::path(
    get,
    path = "/products/{id}",
    operation_id = "get_product",
    params(
        ("id" = String, Path, description = "商品ID", example = "product-123")
    ),
    responses(
        (status = 200, description = "商品詳細の取得成功", body = GetProductResponse),
        (status = 404, description = "商品が見つかりません", body = ErrorResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Products"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
    Path(id): Path<String>
) -> Result<Json<GetProductResponse>> {
    println!("->> GetProductController::handle - product_id: {}", id);
    
    let dispatcher = container.get_dispatcher();
    
    let product_detail = dispatcher
        .execute_get_product_query(GetProductQuery::new(id.clone()))
        .await?; // ApplicationErrorからErrorへの自動変換を利用
        
    println!("->> GetProductController::handle - success for product_id: {}", id);
    Ok(Json(GetProductPresenter::present(product_detail)))
} 