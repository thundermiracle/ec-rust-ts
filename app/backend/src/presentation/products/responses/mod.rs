mod variant_response;
mod get_product_response;
mod get_product_list_response;

// Use Case固有のレスポンス型
pub use get_product_response::GetProductResponse;
pub use get_product_list_response::{GetProductListResponse, GetProductListItemResponse};

// 共通で使用されるレスポンス型
pub use variant_response::VariantResponse;