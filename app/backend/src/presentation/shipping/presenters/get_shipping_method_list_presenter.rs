use crate::application::dto::ShippingMethodListDTO;
use crate::presentation::shipping::responses::{GetShippingMethodListResponse, GetShippingMethodListItemResponse};

/// Shipping Method List Presenter
/// Clean Architecture: Interface Adapters層
/// アプリケーション層のDTOをHTTPレスポンス用DTOに変換する
pub struct GetShippingMethodListPresenter;

impl GetShippingMethodListPresenter {
    /// DTOをResponseに変換
    pub fn present(shipping_method_list: ShippingMethodListDTO) -> GetShippingMethodListResponse {
        let shipping_methods = shipping_method_list
            .methods
            .into_iter()
            .map(|method| GetShippingMethodListItemResponse {
                id: method.id,
                name: method.name,
                description: method.description,
                price: method.price,
            })
            .collect();

        GetShippingMethodListResponse::new(shipping_methods)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_present() {
        use crate::application::dto::ShippingMethodDTO;
        
        let methods = vec![
            ShippingMethodDTO {
                id: "standard".to_string(),
                name: "標準配送".to_string(),
                description: "5-7営業日".to_string(),
                price: 500,
            },
        ];

        let shipping_method_list = ShippingMethodListDTO::new(methods);
        let response = GetShippingMethodListPresenter::present(shipping_method_list);
        
        assert_eq!(response.shipping_methods.len(), 1);
        assert_eq!(response.shipping_methods[0].id, "standard");
        assert_eq!(response.shipping_methods[0].name, "標準配送");
    }
}