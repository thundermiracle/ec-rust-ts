use axum::extract::State;
use axum::{Json, Router, routing::get};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::payment_methods::presenters::GetPaymentMethodListPresenter;
use crate::presentation::payment_methods::responses::GetPaymentMethodListResponse;
use crate::presentation::ErrorResponse;

/// PaymentMethod List Controller
pub struct GetPaymentMethodListController;

impl GetPaymentMethodListController {
    /// ルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/payment-methods", get(handle))
    }
}

/// GET /payment-methods - PaymentMethodリスト取得処理
#[utoipa::path(
    get,
    path = "/payment-methods",
    operation_id = "get_payment_method_list",
    responses(
        (status = 200, description = "PaymentMethodリスト取得成功", body = GetPaymentMethodListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "PaymentMethods"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<GetPaymentMethodListResponse>> {
    println!("->> GetPaymentMethodListController::handle");

    let dispatcher = container.get_dispatcher();
    let result = dispatcher.execute_get_payment_method_list_query().await?;

    println!("->> GetPaymentMethodListController::handle - success");

    Ok(Json(GetPaymentMethodListPresenter::present(result)))
}