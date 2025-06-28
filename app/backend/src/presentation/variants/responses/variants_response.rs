use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct VariantResponse {
    #[serde(rename = "skuId")]
    pub sku_id: String,
    pub price: i32,
    #[serde(rename = "salePrice")]
    #[schema(nullable = false)]
    pub sale_price: Option<i32>,
    #[schema(nullable = false)]
    pub image: Option<String>,
    #[schema(nullable = false)]
    pub material: Option<String>,
    #[schema(nullable = false)]
    pub dimensions: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VariantsResponse {
    pub variants: Vec<VariantResponse>,
}
