use crate::domain::value_objects::{ShippingMethodId, Money, Address};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShippingInfo {
    pub method_id: ShippingMethodId,
    pub method_name: String,
    pub fee: Money,
    pub address: Address,
}

impl ShippingInfo {
    pub fn new(
        method_id: ShippingMethodId,
        method_name: String,
        fee: Money,
        address: Address,
    ) -> Self {
        ShippingInfo {
            method_id,
            method_name,
            fee,
            address,
        }
    }
    
    pub fn formatted_address(&self) -> String {
        self.address.formatted()
    }
    
    pub fn method_id_value(&self) -> &str {
        self.method_id.value()
    }
    
    pub fn fee_amount(&self) -> u32 {
        self.fee.amount_in_yen()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shipping_info() {
        let method_id = ShippingMethodId::new("standard".to_string()).unwrap();
        let method_name = "Standard Shipping".to_string();
        let fee = Money::from_yen(500);
        let address = Address::new(
            "123-4567".to_string(),
            "Tokyo".to_string(),
            "Shibuya".to_string(),
            "1-2-3".to_string(),
            None,
        ).unwrap();
        
        let shipping_info = ShippingInfo::new(method_id, method_name.clone(), fee, address);
        
        assert_eq!(shipping_info.method_name, method_name);
        assert_eq!(shipping_info.fee_amount(), 500);
        assert_eq!(shipping_info.method_id_value(), "standard");
        assert!(!shipping_info.formatted_address().is_empty());
    }
}