use axum::extract::State;
use axum::{Json, Router, routing::get};
use std::sync::Arc;

use crate::error::Result;
use crate::infrastructure::Container;
use crate::presentation::colors::presenters::ColorListPresenter;
use crate::presentation::colors::responses::ColorListResponse;
use crate::presentation::ErrorResponse;

/// Colors Controller - 色一覧取得
pub struct GetColorListController;

impl GetColorListController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new().route("/colors", get(handle))
    }
}


/// GET /colors - 色一覧取得処理
#[utoipa::path(
    get,
    path = "/colors",
    operation_id = "get_color_list",
    responses(
        (status = 200, description = "色一覧の取得成功", body = ColorListResponse),
        (status = 500, description = "内部サーバーエラー", body = ErrorResponse)
    ),
    tag = "Colors"
)]
pub async fn handle(
    State(container): State<Arc<Container>>,
) -> Result<Json<ColorListResponse>> {
    println!("->> GetColorListController::handle");

    let dispatcher = container.get_dispatcher();
    let colors = dispatcher.execute_get_color_list_query().await?;

    println!("->> GetColorListController::handle - success for colors");

    Ok(Json(ColorListPresenter::present(colors)))
}
