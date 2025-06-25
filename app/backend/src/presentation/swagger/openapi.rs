use utoipa::OpenApi;

use crate::presentation::products::responses::{
    ProductResponse, 
    ProductListResponse, 
    ProductListItemResponse, 
    VariantResponse
};
use crate::presentation::categories::responses::{
    CategoryListResponse,
    CategoryResponse
};
use crate::presentation::colors::{
    ColorListResponse,
    ColorListItemResponse
};
use crate::presentation::ErrorResponse;

/// OpenAPI仕様書の定義
/// Clean Architecture: Interface Adapters層でAPI仕様を定義
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::products::controllers::get_product_controller::handle,
        crate::presentation::products::controllers::get_product_list_controller::handle,
        crate::presentation::categories::controllers::get_categorie_list_controller::handle,
        crate::presentation::colors::controllers::get_color_list_controller::handle,
    ),
    components(
        schemas(
            ProductResponse,
            ProductListResponse,
            ProductListItemResponse,
            VariantResponse,
            CategoryListResponse,
            CategoryResponse,
            ColorListResponse,
            ColorListItemResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Products", description = "商品関連のAPI"),
        (name = "Categories", description = "カテゴリ関連のAPI"),
        (name = "Colors", description = "色関連のAPI")
    ),
    info(
        title = "ECサイト API",
        description = "Clean Architectureを適用したECサイトのREST API",
        version = "1.0.0",
        contact(
            name = "Development Team",
            email = "dev@example.com"
        )
    ),
    servers(
        (url = "http://localhost:4000", description = "Development server"),
        (url = "https://api.example.com", description = "Production server")
    )
)]
pub struct ApiDoc; 