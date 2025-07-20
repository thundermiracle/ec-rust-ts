use axum::{Json, Router, extract::State, routing::post};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::ErrorResponse;
use crate::presentation::common::extractors::ValidatedJson;
use crate::presentation::orders::{CreateOrderRequest, CreateOrderResponse, OrderPresenter};

pub struct CreateOrderController;

impl CreateOrderController {
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/orders", post(handle))
    }
}

/// POST /orders - 注文作成処理
/// 顧客情報、商品、配送・支払い情報から注文を作成する
#[utoipa::path(
    post,
    path = "/orders",
    operation_id = "create_order",
    request_body = CreateOrderRequest,
    responses(
        (status = 201, description = "注文作成成功", body = CreateOrderResponse),
        (status = 400, description = "リクエストが無効です", body = ErrorResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Orders"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
    ValidatedJson(request): ValidatedJson<CreateOrderRequest>,
) -> Result<Json<CreateOrderResponse>> {
    println!(
        "->> CreateOrderController::handle - {} items for {}",
        request.items.len(),
        request.customer_info.email
    );

    // 1. バリデーションはValidatedJsonエクストラクターで完了

    // 2. アプリケーション層のコマンドに変換
    let command = request.to_command();

    // 3. Dispatcherを通じてユースケースを実行
    let dispatcher = container.get_dispatcher();
    let result = dispatcher.execute_create_order_command(command).await?;

    // 4. プレゼンターでレスポンスに変換
    let response = OrderPresenter::to_response(result);

    println!(
        "->> CreateOrderController::handle - success, order_id: {}",
        response.order_id
    );
    Ok(Json(response))
}