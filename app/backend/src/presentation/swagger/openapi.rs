use utoipa::OpenApi;

use crate::presentation::products::responses::{
    GetProductResponse, 
    GetProductListResponse, 
    GetProductListItemResponse, 
    VariantResponse
};
use crate::presentation::categories::responses::{
    GetCategoryListResponse,
    CategoryResponse
};
use crate::presentation::colors::responses::{
    GetColorListResponse,
    GetColorListItemResponse
};
use crate::presentation::variants::responses::{
    FindVariantsResponse,
    FindVariantsItemResponse
};
use crate::presentation::variants::requests::FindVariantsRequest;
use crate::presentation::cart::requests::{CalculateCartRequest, CalculateCartItemRequest};
use crate::presentation::cart::responses::{CalculateCartResponse, CalculateCartItemResponse};
use crate::presentation::shipping::responses::{
    GetShippingMethodListResponse,
    GetShippingMethodListItemResponse
};
use crate::presentation::ErrorResponse;

/// OpenAPI仕様書の定義
/// Clean Architecture: Interface Adapters層でAPI仕様を定義
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::presentation::products::controllers::get_product_controller::handle,
        crate::presentation::products::controllers::get_product_list_controller::handle,
        crate::presentation::categories::controllers::get_category_list_controller::handle,
        crate::presentation::colors::controllers::get_color_list_controller::handle,
        crate::presentation::variants::controllers::find_variants_controller::handle,
        crate::presentation::cart::controllers::calculate_cart_controller::handle,
        crate::presentation::shipping::controllers::get_shipping_method_list_controller::handle,
    ),
    components(
        schemas(
            GetProductResponse,
            GetProductListResponse,
            GetProductListItemResponse,
            VariantResponse,
            GetCategoryListResponse,
            CategoryResponse,
            GetColorListResponse,
            GetColorListItemResponse,
            FindVariantsResponse,
            FindVariantsItemResponse,
            FindVariantsRequest,
            CalculateCartRequest,
            CalculateCartItemRequest,
            CalculateCartResponse,
            CalculateCartItemResponse,
            GetShippingMethodListResponse,
            GetShippingMethodListItemResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Products", description = "商品関連のAPI"),
        (name = "Categories", description = "カテゴリ関連のAPI"),
        (name = "Colors", description = "色関連のAPI"),
        (name = "Variants", description = "バリアント関連のAPI"),
        (name = "Shipping", description = "配送関連のAPI")
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