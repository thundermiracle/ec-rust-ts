use utoipa::OpenApi;

use crate::presentation::ErrorResponse;
use crate::presentation::cart::requests::{CalculateCartItemRequest, CalculateCartRequest};
use crate::presentation::cart::responses::{CalculateCartItemResponse, CalculateCartResponse};
use crate::presentation::categories::responses::{CategoryResponse, GetCategoryListResponse};
use crate::presentation::colors::responses::{GetColorListItemResponse, GetColorListResponse};
use crate::presentation::payment_methods::responses::{
    GetPaymentMethodListResponse, PaymentMethodListItemResponse,
};
use crate::presentation::products::responses::{
    GetProductListItemResponse, GetProductListResponse, GetProductResponse, VariantResponse,
};
use crate::presentation::shipping::responses::{
    GetShippingMethodListItemResponse, GetShippingMethodListResponse,
};
use crate::presentation::variants::requests::FindVariantsRequest;
use crate::presentation::variants::responses::{FindVariantsItemResponse, FindVariantsResponse};

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
        crate::presentation::payment_methods::controllers::get_payment_method_list_controller::handle,
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
            GetPaymentMethodListResponse,
            PaymentMethodListItemResponse,
            ErrorResponse
        )
    ),
    tags(
        (name = "Products", description = "商品関連のAPI"),
        (name = "Categories", description = "カテゴリ関連のAPI"),
        (name = "Colors", description = "色関連のAPI"),
        (name = "Variants", description = "バリアント関連のAPI"),
        (name = "Shipping", description = "配送関連のAPI"),
        (name = "PaymentMethods", description = "支払い方法関連のAPI")
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
