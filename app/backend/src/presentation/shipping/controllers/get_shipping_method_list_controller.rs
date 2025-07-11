use axum::extract::State;
use axum::{Json, Router, routing::get};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::shipping::presenters::GetShippingMethodListPresenter;
use crate::presentation::shipping::responses::GetShippingMethodListResponse;
use crate::presentation::ErrorResponse;

/// Shipping Method List Controller - 配送方法一覧取得
pub struct GetShippingMethodListController;

impl GetShippingMethodListController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/shipping-methods", get(handle))
    }
}

/// GET /shipping-methods - 配送方法一覧取得処理
/// 
/// アクティブな配送方法をすべて取得し、sort_order順で返す
#[utoipa::path(
    get,
    path = "/shipping-methods",
    operation_id = "get_shipping_method_list",
    responses(
        (status = 200, description = "配送方法一覧の取得成功", body = GetShippingMethodListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Shipping"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<GetShippingMethodListResponse>> {
    println!("->> GetShippingMethodListController::handle");
    
    let dispatcher = container.get_dispatcher();
    let shipping_methods = dispatcher.execute_get_shipping_method_list_query().await?;

    println!("->> GetShippingMethodListController::handle - success for shipping methods");

    Ok(Json(GetShippingMethodListPresenter::present(shipping_methods)))
}