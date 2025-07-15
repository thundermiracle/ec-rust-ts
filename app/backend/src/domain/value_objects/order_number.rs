use crate::domain::error::DomainError;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderNumber(String);

impl OrderNumber {
    const MAX_LENGTH: usize = 20;
    
    pub fn generate(year: i32, sequence: u32) -> Self {
        Self(format!("ORD-{}-{:06}", year, sequence))
    }
    
    pub fn from_string(value: String) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Order number cannot be empty".to_string(),
            ));
        }
        
        if value.len() > Self::MAX_LENGTH {
            return Err(DomainError::InvalidProductData(
                format!("Order number cannot exceed {} characters", Self::MAX_LENGTH),
            ));
        }
        
        // Basic format validation for generated order numbers
        if value.starts_with("ORD-") && value.len() == 15 {
            // Validate format: ORD-YYYY-NNNNNN
            let parts: Vec<&str> = value.split('-').collect();
            if parts.len() == 3 {
                if let (Ok(_year), Ok(_seq)) = (parts[1].parse::<i32>(), parts[2].parse::<u32>()) {
                    return Ok(Self(value));
                }
            }
        }
        
        // Allow custom order numbers that don't follow the standard format
        Ok(Self(value))
    }
    
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for OrderNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_order_number() {
        let order_number = OrderNumber::generate(2024, 123);
        assert_eq!(order_number.value(), "ORD-2024-000123");
    }

    #[test]
    fn test_from_string_valid() {
        let order_number = OrderNumber::from_string("ORD-2024-000001".to_string()).unwrap();
        assert_eq!(order_number.value(), "ORD-2024-000001");
    }

    #[test]
    fn test_from_string_custom_format() {
        let order_number = OrderNumber::from_string("CUSTOM-001".to_string()).unwrap();
        assert_eq!(order_number.value(), "CUSTOM-001");
    }

    #[test]
    fn test_from_string_empty() {
        let result = OrderNumber::from_string("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_from_string_too_long() {
        let long_string = "A".repeat(21);
        let result = OrderNumber::from_string(long_string);
        assert!(result.is_err());
    }
}