use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::ErrorResponse;
use crate::presentation::cart::{CalculateCartRequest, CalculateCartResponse, CartPresenter};

pub struct CalculateCartController;

impl CalculateCartController {
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/cart", post(handle))
    }
}

/// POST /cart - カート計算処理
/// カートの各アイテムの価格と総額を計算して返す
#[utoipa::path(
    post,
    path = "/cart",
    operation_id = "calculate_cart",
    request_body = CalculateCartRequest,
    responses(
        (status = 200, description = "カート計算成功", body = CalculateCartResponse),
        (status = 400, description = "リクエストが無効です", body = ErrorResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Cart"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
    Json(request): Json<CalculateCartRequest>,
) -> Result<Json<CalculateCartResponse>> {
    println!(
        "->> CalculateCartController::handle - {} items",
        request.items.len()
    );

    // 1. リクエストバリデーション
    request
        .validate()
        .map_err(|msg| crate::application::error::ApplicationError::Validation(msg))?;

    // 2. アプリケーション層のコマンドに変換
    let command = request.to_command();

    // 3. Dispatcherを通じてユースケースを実行
    let dispatcher = container.get_dispatcher();
    let result = dispatcher.execute_calculate_cart_command(command).await?; // ApplicationErrorからErrorへの自動変換を利用

    // 4. プレゼンターでレスポンスに変換
    let response = CartPresenter::to_response(result);

    println!(
        "->> CalculateCartController::handle - success for cart with {} items",
        response.item_count
    );
    Ok(Json(response))
}
