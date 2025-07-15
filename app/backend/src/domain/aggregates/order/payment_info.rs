use crate::domain::value_objects::{PaymentMethodId, Money};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentDetails {
    pub details: String, // JSON string instead of serde_json::Value
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaymentInfo {
    pub method_id: PaymentMethodId,
    pub method_name: String,
    pub fee: Money,
    pub payment_details: Option<PaymentDetails>,
}

impl PaymentInfo {
    pub fn new(
        method_id: PaymentMethodId,
        method_name: String,
        fee: Money,
        payment_details: Option<PaymentDetails>,
    ) -> Self {
        PaymentInfo {
            method_id,
            method_name,
            fee,
            payment_details,
        }
    }
    
    pub fn method_id_value(&self) -> uuid::Uuid {
        self.method_id.value()
    }
    
    pub fn fee_amount(&self) -> u32 {
        self.fee.amount_in_yen()
    }
    
    pub fn has_details(&self) -> bool {
        self.payment_details.is_some()
    }
}

impl PaymentDetails {
    pub fn new(details: String) -> Self {
        PaymentDetails { details }
    }
    
    pub fn from_json_string(json_str: &str) -> Self {
        PaymentDetails { details: json_str.to_string() }
    }
    
    pub fn to_json_string(&self) -> &str {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_info() {
        let method_id = PaymentMethodId::new();
        let method_name = "Credit Card".to_string();
        let fee = Money::from_yen(100);
        let details = PaymentDetails::new(r#"{"card_type": "visa", "last_four": "1234"}"#.to_string());
        
        let payment_info = PaymentInfo::new(method_id, method_name.clone(), fee, Some(details));
        
        assert_eq!(payment_info.method_name, method_name);
        assert_eq!(payment_info.fee_amount(), 100);
        assert!(payment_info.has_details());
    }

    #[test]
    fn test_payment_info_without_details() {
        let method_id = PaymentMethodId::new();
        let method_name = "Cash on Delivery".to_string();
        let fee = Money::from_yen(300);
        
        let payment_info = PaymentInfo::new(method_id, method_name, fee, None);
        
        assert!(!payment_info.has_details());
    }

    #[test]
    fn test_payment_details() {
        let details_json = r#"{"card_type": "mastercard", "expiry": "12/25"}"#;
        
        let details = PaymentDetails::new(details_json.to_string());
        
        assert_eq!(details.details, details_json);
        
        let json_string = details.to_json_string();
        let restored_details = PaymentDetails::from_json_string(json_string);
        
        assert_eq!(details.details, restored_details.details);
    }
}