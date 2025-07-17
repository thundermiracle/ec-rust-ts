use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Get Shipping Method List Response DTO
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetShippingMethodListResponse {
    pub shipping_methods: Vec<GetShippingMethodListItemResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetShippingMethodListItemResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u32,
}

impl GetShippingMethodListResponse {
    pub fn new(shipping_methods: Vec<GetShippingMethodListItemResponse>) -> Self {
        Self { shipping_methods }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_shipping_method_list_response_serialization() {
        let methods = vec![
            GetShippingMethodListItemResponse {
                id: "standard".to_string(),
                name: "標準配送".to_string(),
                description: "5-7営業日".to_string(),
                price: 500,
            },
            GetShippingMethodListItemResponse {
                id: "express".to_string(),
                name: "速達配送".to_string(),
                description: "2-3営業日".to_string(),
                price: 1000,
            },
        ];

        let response = GetShippingMethodListResponse::new(methods);
        let json = serde_json::to_string(&response).unwrap();

        assert!(json.contains("standard"));
        assert!(json.contains("標準配送"));
        assert!(json.contains("5-7営業日"));
        assert!(json.contains("500"));
    }
}
