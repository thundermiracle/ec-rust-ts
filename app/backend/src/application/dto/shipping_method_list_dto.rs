use serde::{Serialize, Deserialize};
use crate::domain::entities::ShippingMethod;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethodListDTO {
    pub methods: Vec<ShippingMethodDTO>,
}

impl ShippingMethodListDTO {
    pub fn new(methods: Vec<ShippingMethodDTO>) -> Self {
        Self { methods }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingMethodDTO {
    pub id: String,
    pub name: String,
    pub description: String,
    pub price: u32,
}

impl From<ShippingMethod> for ShippingMethodDTO {
    fn from(method: ShippingMethod) -> Self {
        Self {
            id: method.id().value().to_string(),
            name: method.name().to_string(),
            description: method.description().to_string(),
            price: method.price().yen(),
        }
    }
}

impl From<&ShippingMethod> for ShippingMethodDTO {
    fn from(method: &ShippingMethod) -> Self {
        Self {
            id: method.id().value().to_string(),
            name: method.name().to_string(),
            description: method.description().to_string(),
            price: method.price().yen(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{ShippingMethodId, Money};

    #[test]
    fn test_shipping_method_dto_from_entity() {
        let id = ShippingMethodId::new("standard".to_string()).unwrap();
        let method = ShippingMethod::new(
            id,
            "標準配送".to_string(),
            "5-7営業日".to_string(),
            Money::from_yen(500),
            true,
            1,
        );

        let dto: ShippingMethodDTO = method.into();

        assert_eq!(dto.id, "standard");
        assert_eq!(dto.name, "標準配送");
        assert_eq!(dto.description, "5-7営業日");
        assert_eq!(dto.price, 500);
    }

    #[test]
    fn test_shipping_method_dto_from_reference() {
        let id = ShippingMethodId::new("express".to_string()).unwrap();
        let method = ShippingMethod::new(
            id,
            "速達配送".to_string(),
            "2-3営業日".to_string(),
            Money::from_yen(1000),
            true,
            2,
        );

        let dto: ShippingMethodDTO = (&method).into();

        assert_eq!(dto.id, "express");
        assert_eq!(dto.name, "速達配送");
        assert_eq!(dto.description, "2-3営業日");
        assert_eq!(dto.price, 1000);
    }
}