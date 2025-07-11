use crate::domain::value_objects::{ShippingMethodId, Money};
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct ShippingMethod {
    id: ShippingMethodId,
    name: String,
    description: String,
    price: Money,
    is_active: bool,
    sort_order: u32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl ShippingMethod {
    pub fn new(
        id: ShippingMethodId,
        name: String,
        description: String,
        price: Money,
        is_active: bool,
        sort_order: u32,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            price,
            is_active,
            sort_order,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn with_timestamps(
        id: ShippingMethodId,
        name: String,
        description: String,
        price: Money,
        is_active: bool,
        sort_order: u32,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            is_active,
            sort_order,
            created_at,
            updated_at,
        }
    }

    // Getters
    pub fn id(&self) -> &ShippingMethodId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn price(&self) -> &Money {
        &self.price
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn sort_order(&self) -> u32 {
        self.sort_order
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

impl fmt::Display for ShippingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {} ({})",
            self.name,
            self.description,
            self.price
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::error::DomainError;

    #[test]
    fn test_new_shipping_method() -> Result<(), DomainError> {
        let id = ShippingMethodId::new("standard".to_string())?;
        let name = "標準配送".to_string();
        let description = "5-7営業日".to_string();
        let price = Money::from_yen(500);

        let method = ShippingMethod::new(id, name, description, price, true, 1);

        assert_eq!(method.name(), "標準配送");
        assert_eq!(method.description(), "5-7営業日");
        assert_eq!(method.price().yen(), 500);
        assert!(method.is_active());
        assert_eq!(method.sort_order(), 1);

        Ok(())
    }

    #[test]
    fn test_display() -> Result<(), DomainError> {
        let id = ShippingMethodId::new("express".to_string())?;
        let name = "速達配送".to_string();
        let description = "2-3営業日".to_string();
        let price = Money::from_yen(1000);

        let method = ShippingMethod::new(id, name, description, price, true, 2);
        let display_str = format!("{}", method);

        assert_eq!(display_str, "速達配送 - 2-3営業日 (¥1000)");

        Ok(())
    }
}