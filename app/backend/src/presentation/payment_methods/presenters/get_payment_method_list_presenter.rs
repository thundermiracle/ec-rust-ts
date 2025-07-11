use crate::application::dto::PaymentMethodListDTO;
use crate::presentation::payment_methods::responses::{GetPaymentMethodListResponse, PaymentMethodListItemResponse};

/// PaymentMethodリストプレゼンター
/// Clean Architecture: Interface Adapters層
/// アプリケーション層のDTOをHTTPレスポンス用DTOに変換する
pub struct GetPaymentMethodListPresenter;

impl GetPaymentMethodListPresenter {
    /// DTOをResponseに変換
    pub fn present(dto: PaymentMethodListDTO) -> GetPaymentMethodListResponse {
        let items = dto
            .items
            .into_iter()
            .map(|item| PaymentMethodListItemResponse {
                id: item.id,
                name: Some(item.name),
                description: Some(item.description),
            })
            .collect();

        GetPaymentMethodListResponse::new(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{PaymentMethodListDTO, PaymentMethodDTO};

    #[test]
    fn test_present() {
        let dto = PaymentMethodListDTO::new(vec![
            PaymentMethodDTO {
                id: "1".to_string(),
                name: "Test Item".to_string(),
                description: "Test Description".to_string(),
            },
        ]);

        let response = GetPaymentMethodListPresenter::present(dto);
        
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "1");
        assert_eq!(response.items[0].name, Some("Test Item".to_string()));
        assert_eq!(response.items[0].description, Some("Test Description".to_string()));
    }
}