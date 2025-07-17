use crate::domain::error::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Dimensions(String);

impl Dimensions {
    pub fn new(dimensions: String) -> Result<Self, DomainError> {
        let trimmed = dimensions.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Dimensions cannot be empty".to_string(),
            ));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::InvalidProductData(
                "Dimensions cannot exceed 100 characters".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Material(String);

impl Material {
    pub fn new(material: String) -> Result<Self, DomainError> {
        let trimmed = material.trim();
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Material cannot be empty".to_string(),
            ));
        }
        if trimmed.len() > 100 {
            return Err(DomainError::InvalidProductData(
                "Material cannot exceed 100 characters".to_string(),
            ));
        }
        Ok(Self(trimmed.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn abbreviated(&self) -> &str {
        match self.0.as_str() {
            "Oak Wood" => "OAK",
            "Walnut Wood" => "WAL",
            "Natural Bamboo" => "BAM",
            "White Oak" => "WOAK",
            "Pine Wood" => "PINE",
            _ => {
                if self.0.len() >= 3 {
                    &self.0[..3]
                } else {
                    &self.0
                }
            }
        }
    }
}
