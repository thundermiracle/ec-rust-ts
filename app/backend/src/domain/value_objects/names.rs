use crate::domain::error::DomainError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProductName(String);

impl ProductName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductName(
                "Product name cannot be empty".to_string(),
            ));
        }
        if trimmed.len() > 255 {
            return Err(DomainError::InvalidProductName(
                "Product name cannot exceed 255 characters".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ProductName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SKUName(String);

impl SKUName {
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed = name.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "SKU name cannot be empty".to_string(),
            ));
        }
        if trimmed.len() > 255 {
            return Err(DomainError::InvalidProductData(
                "SKU name cannot exceed 255 characters".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SKUCode(String);

impl SKUCode {
    pub fn new(code: String) -> Result<Self, DomainError> {
        let trimmed = code.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidSKUCode(
                "SKU code cannot be empty".to_string(),
            ));
        }
        if trimmed.len() > 50 {
            return Err(DomainError::InvalidSKUCode(
                "SKU code cannot exceed 50 characters".to_string(),
            ));
        }

        if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err(DomainError::InvalidSKUCode(
                "SKU code can only contain alphanumeric characters, hyphens, and underscores".to_string(),
            ));
        }

        Ok(Self(trimmed.to_uppercase()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SKUCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
} 