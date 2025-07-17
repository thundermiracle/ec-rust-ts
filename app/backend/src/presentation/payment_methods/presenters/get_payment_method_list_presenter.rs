use crate::application::dto::PaymentMethodListDTO;
use crate::presentation::payment_methods::responses::{
    GetPaymentMethodListResponse, PaymentMethodListItemResponse,
};

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
    use crate::application::dto::{PaymentMethodDTO, PaymentMethodListDTO};

    #[test]
    fn test_present() {
        let dto = PaymentMethodListDTO::new(vec![
            PaymentMethodDTO {
                id: "credit_card".to_string(),
                name: "クレジットカード".to_string(),
                description: "VISA、MasterCard、JCB対応".to_string(),
            },
            PaymentMethodDTO {
                id: "cod".to_string(),
                name: "代引き".to_string(),
                description: "商品到着時に現金でお支払い".to_string(),
            },
        ]);

        let response = GetPaymentMethodListPresenter::present(dto);

        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].id, "credit_card");
        assert_eq!(response.items[0].name, Some("クレジットカード".to_string()));
        assert_eq!(
            response.items[0].description,
            Some("VISA、MasterCard、JCB対応".to_string())
        );

        assert_eq!(response.items[1].id, "cod");
        assert_eq!(response.items[1].name, Some("代引き".to_string()));
        assert_eq!(
            response.items[1].description,
            Some("商品到着時に現金でお支払い".to_string())
        );
    }

    #[test]
    fn test_present_empty_list() {
        let dto = PaymentMethodListDTO::new(vec![]);
        let response = GetPaymentMethodListPresenter::present(dto);

        assert_eq!(response.items.len(), 0);
    }

    #[test]
    fn test_present_with_all_payment_methods() {
        let dto = PaymentMethodListDTO::new(vec![
            PaymentMethodDTO {
                id: "credit_card".to_string(),
                name: "クレジットカード".to_string(),
                description: "VISA、MasterCard、JCB対応".to_string(),
            },
            PaymentMethodDTO {
                id: "cod".to_string(),
                name: "代引き".to_string(),
                description: "商品到着時に現金でお支払い".to_string(),
            },
            PaymentMethodDTO {
                id: "bank_transfer".to_string(),
                name: "銀行振込".to_string(),
                description: "指定口座への事前振込".to_string(),
            },
            PaymentMethodDTO {
                id: "convenience_store".to_string(),
                name: "コンビニ支払い".to_string(),
                description: "セブンイレブン、ファミリーマート等".to_string(),
            },
        ]);

        let response = GetPaymentMethodListPresenter::present(dto);

        assert_eq!(response.items.len(), 4);

        // Test all payment method types are correctly presented
        let payment_methods: Vec<String> =
            response.items.iter().map(|item| item.id.clone()).collect();
        assert!(payment_methods.contains(&"credit_card".to_string()));
        assert!(payment_methods.contains(&"cod".to_string()));
        assert!(payment_methods.contains(&"bank_transfer".to_string()));
        assert!(payment_methods.contains(&"convenience_store".to_string()));
    }
}
