use crate::presentation::products::routes as products_routes;
use crate::presentation::categories::routes as categories_routes;
use crate::presentation::colors::routes as colors_routes;
use crate::presentation::variants::routes as variants_routes;
use crate::presentation::swagger::swagger_routes;

use axum::Router;
use std::sync::Arc;
use crate::infrastructure::Container;

pub fn routes() -> Router<Arc<Container>> {
    Router::new()
        .merge(products_routes())
        .merge(categories_routes())
        .merge(colors_routes())
        .merge(variants_routes())
        .merge(swagger_routes())        // Swagger UI + OpenAPI JSON
}