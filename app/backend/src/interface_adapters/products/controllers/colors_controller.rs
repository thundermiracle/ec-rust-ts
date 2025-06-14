use axum::{routing::get, Json, Router};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::frameworks_and_drivers::Container;
use crate::error::Result;

/// Color Response DTO
#[derive(Serialize, Deserialize)]
pub struct ColorResponse {
    pub id: String,
    pub name: String,
    pub hex_code: Option<String>,
    pub display_order: u32,
    pub product_count: u32,
}

/// Colors Controller - 色一覧取得
pub struct ColorsController;

impl ColorsController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/colors", get(Self::handle_list))
    }

    /// GET /colors - 色一覧取得処理
    async fn handle_list(
        // State(container): State<Arc<Container>>,
    ) -> Result<Json<Vec<ColorResponse>>> {
        // TODO: 後でデータベースから取得するように変更
        // 現在はモックデータを返す
        let colors = vec![
            ColorResponse {
                id: "walnut".to_string(),
                name: "Walnut".to_string(),
                hex_code: Some("#8B4513".to_string()),
                display_order: 1,
                product_count: 4,
            },
            ColorResponse {
                id: "white-oak".to_string(),
                name: "White Oak".to_string(),
                hex_code: Some("#F5DEB3".to_string()),
                display_order: 2,
                product_count: 1,
            },
            ColorResponse {
                id: "black-oak".to_string(),
                name: "Black Oak".to_string(),
                hex_code: Some("#2F2F2F".to_string()),
                display_order: 3,
                product_count: 1,
            },
            ColorResponse {
                id: "brass".to_string(),
                name: "Brass".to_string(),
                hex_code: Some("#B5651D".to_string()),
                display_order: 4,
                product_count: 1,
            },
            ColorResponse {
                id: "charcoal".to_string(),
                name: "Charcoal".to_string(),
                hex_code: Some("#36454F".to_string()),
                display_order: 5,
                product_count: 1,
            },
            ColorResponse {
                id: "upholstered".to_string(),
                name: "Upholstered".to_string(),
                hex_code: Some("#D2B48C".to_string()),
                display_order: 6,
                product_count: 1,
            },
        ];
        
        Ok(Json(colors))
    }
} 