use axum::extract::State;
use axum::{Json, Router, routing::post};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::ErrorResponse;
use crate::presentation::variants::presenters::FindVariantsPresenter;
use crate::presentation::variants::requests::FindVariantsRequest;
use crate::presentation::variants::responses::FindVariantsResponse;

/// Get Variants Controller - バリアント詳細取得の単一責任
/// Clean Architecture: 1つのユースケースに対して1つのController
pub struct FindVariantsController;

impl FindVariantsController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/variants", post(handle))
    }
}

/// POST /variants - バリアント詳細取得処理
/// SKU IDの配列を受け取り、価格や材質等の情報を返す
#[utoipa::path(
    post,
    path = "/variants",
    operation_id = "find_variants",
    request_body = FindVariantsRequest,
    responses(
        (status = 200, description = "バリアント詳細の取得成功", body = FindVariantsResponse),
        (status = 400, description = "リクエストが無効です", body = ErrorResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Variants"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
    Json(request): Json<FindVariantsRequest>,
) -> Result<Json<FindVariantsResponse>> {
    println!(
        "->> FindVariantsController::handle - sku_ids: {:?}",
        request.sku_ids
    );

    // リクエストからクエリを作成
    let query = request.to_query().map_err(|_| {
        crate::application::error::ApplicationError::Validation("Invalid SKU ID format".to_string())
    })?;

    let dispatcher = container.get_dispatcher();

    let variants = dispatcher.execute_find_variants_query(query).await?;

    println!(
        "->> FindVariantsController::handle - success for {} variants",
        variants.len()
    );
    Ok(Json(FindVariantsPresenter::present(variants)))
}
