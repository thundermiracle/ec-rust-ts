use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetPaymentMethodListResponse {
    pub items: Vec<PaymentMethodListItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodListItemResponse {
    pub id: String,
    #[schema(nullable = false)]
    pub name: Option<String>,
    #[schema(nullable = false)]
    pub description: Option<String>,
}

impl GetPaymentMethodListResponse {
    pub fn new(items: Vec<PaymentMethodListItemResponse>) -> Self {
        Self { items }
    }
}