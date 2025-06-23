use axum::{routing::get, Json, Router};
use std::sync::Arc;
use serde::{Deserialize, Serialize};

use crate::infrastructure::Container;
use crate::error::Result;

/// Category Response DTO
#[derive(Serialize, Deserialize)]
pub struct CategoryResponse {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub product_count: u32,
}

/// Categories Controller - カテゴリー一覧取得
pub struct CategoriesController;

impl CategoriesController {
    /// このControllerのルート定義
    pub fn routes() -> Router<Arc<Container>> {
        Router::new()
            .route("/categories", get(Self::handle_list))
    }

    /// GET /categories - カテゴリー一覧取得処理
    async fn handle_list(
        // State(container): State<Arc<Container>>,
    ) -> Result<Json<Vec<CategoryResponse>>> {
        // TODO: 後でデータベースから取得するように変更
        // 現在はモックデータを返す
        let categories = vec![
            CategoryResponse {
                id: "desks".to_string(),
                name: "Desks".to_string(),
                slug: "desks".to_string(),
                description: Some("Work desks and office furniture".to_string()),
                product_count: 3,
            },
            CategoryResponse {
                id: "chairs".to_string(),
                name: "Chairs".to_string(),
                slug: "chairs".to_string(),
                description: Some("Office and dining chairs".to_string()),
                product_count: 2,
            },
            CategoryResponse {
                id: "tables".to_string(),
                name: "Tables".to_string(),
                slug: "tables".to_string(),
                description: Some("Coffee tables and dining tables".to_string()),
                product_count: 2,
            },
            CategoryResponse {
                id: "lighting".to_string(),
                name: "Lighting".to_string(),
                slug: "lighting".to_string(),
                description: Some("Pendant lights and lamps".to_string()),
                product_count: 1,
            },
            CategoryResponse {
                id: "storage".to_string(),
                name: "Storage".to_string(),
                slug: "storage".to_string(),
                description: Some("Shelves and storage solutions".to_string()),
                product_count: 1,
            },
            CategoryResponse {
                id: "accessories".to_string(),
                name: "Accessories".to_string(),
                slug: "accessories".to_string(),
                description: Some("Small accessories and stands".to_string()),
                product_count: 1,
            },
        ];
        
        Ok(Json(categories))
    }
} 