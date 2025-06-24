use axum::{Router, routing::get};
use std::sync::Arc;
use utoipa::OpenApi;

use crate::infrastructure::Container;
use crate::presentation::swagger::openapi::ApiDoc;

/// SwaggerUIホスティングとOpenAPI JSONエンドポイント
/// Clean Architecture: Interface Adapters層でSwagger UI関連のエンドポイントを提供
pub fn swagger_routes() -> Router<Arc<Container>> {
    Router::new()
        .route("/api-docs/openapi.json", get(|| async {
            axum::Json(ApiDoc::openapi())
        }))
} 