use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub building: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum AddressError {
    Empty,
    InvalidPostalCode,
    TooLong,
}

impl Address {
    pub fn new(
        postal_code: String,
        prefecture: String,
        city: String,
        street: String,
        building: Option<String>,
    ) -> Result<Self, AddressError> {
        Self::validate_postal_code(&postal_code)?;
        Self::validate_field(&prefecture, "都道府県")?;
        Self::validate_field(&city, "市区町村")?;
        Self::validate_field(&street, "番地")?;
        
        if let Some(ref building) = building {
            Self::validate_field(building, "建物名")?;
        }

        Ok(Self {
            postal_code,
            prefecture,
            city,
            street,
            building,
        })
    }

    pub fn postal_code(&self) -> &str {
        &self.postal_code
    }

    pub fn prefecture(&self) -> &str {
        &self.prefecture
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn building(&self) -> Option<&str> {
        self.building.as_deref()
    }

    pub fn full_address(&self) -> String {
        format!(
            "〒{} {} {} {} {}",
            self.postal_code,
            self.prefecture,
            self.city,
            self.street,
            self.building.as_ref().map(|s| s.as_str()).unwrap_or("")
        ).trim_end().to_string()
    }

    /// フォーマット済み住所（full_addressの別名）
    pub fn formatted(&self) -> String {
        self.full_address()
    }

    fn validate_postal_code(postal_code: &str) -> Result<(), AddressError> {
        if postal_code.trim().is_empty() {
            return Err(AddressError::Empty);
        }

        // 郵便番号の形式チェック (例: 123-4567)
        let re = Regex::new(r"^\d{3}-\d{4}$").unwrap();
        if !re.is_match(postal_code) {
            return Err(AddressError::InvalidPostalCode);
        }

        Ok(())
    }

    fn validate_field(field: &str, _field_name: &str) -> Result<(), AddressError> {
        if field.trim().is_empty() {
            return Err(AddressError::Empty);
        }

        if field.len() > 100 {
            return Err(AddressError::TooLong);
        }

        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_address())
    }
}

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AddressError::Empty => write!(f, "住所の項目が入力されていません"),
            AddressError::InvalidPostalCode => write!(f, "郵便番号の形式が正しくありません（例: 123-4567）"),
            AddressError::TooLong => write!(f, "住所の項目が長すぎます（100文字以内）"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_address() {
        let address = Address::new(
            "123-4567".to_string(),
            "東京都".to_string(),
            "渋谷区".to_string(),
            "渋谷1-1-1".to_string(),
            Some("渋谷ビル".to_string()),
        );
        assert!(address.is_ok());
    }

    #[test]
    fn test_invalid_postal_code() {
        let address = Address::new(
            "12345678".to_string(),
            "東京都".to_string(),
            "渋谷区".to_string(),
            "渋谷1-1-1".to_string(),
            None,
        );
        assert_eq!(address, Err(AddressError::InvalidPostalCode));
    }

    #[test]
    fn test_full_address() {
        let address = Address::new(
            "123-4567".to_string(),
            "東京都".to_string(),
            "渋谷区".to_string(),
            "渋谷1-1-1".to_string(),
            Some("渋谷ビル".to_string()),
        ).unwrap();
        
        assert_eq!(address.full_address(), "〒123-4567 東京都 渋谷区 渋谷1-1-1 渋谷ビル");
    }
}
