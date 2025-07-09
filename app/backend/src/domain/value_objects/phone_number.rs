use regex::Regex;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhoneNumber {
    value: String,
}

#[derive(Debug, PartialEq)]
pub enum PhoneNumberError {
    Empty,
    InvalidFormat,
    TooLong,
}

impl PhoneNumber {
    pub fn new(value: String) -> Result<Self, PhoneNumberError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn validate(value: &str) -> Result<(), PhoneNumberError> {
        if value.is_empty() {
            return Err(PhoneNumberError::Empty);
        }

        if value.len() > 20 {
            return Err(PhoneNumberError::TooLong);
        }

        // 日本の電話番号形式をサポート
        // 固定電話: 0X-XXXX-XXXX または 0XXXXXXXXX
        // 携帯電話: 0X0-XXXX-XXXX または 0X0XXXXXXXX
        let re = Regex::new(r"^(0\d{1,4}-\d{1,4}-\d{1,4}|0\d{9,10})$").unwrap();
        if !re.is_match(value) {
            return Err(PhoneNumberError::InvalidFormat);
        }

        Ok(())
    }

    pub fn formatted(&self) -> String {
        let clean = self.value.replace("-", "");
        
        match clean.len() {
            10 => format!("{}-{}-{}", &clean[0..2], &clean[2..6], &clean[6..10]),
            11 => format!("{}-{}-{}", &clean[0..3], &clean[3..7], &clean[7..11]),
            _ => self.value.clone(),
        }
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.formatted())
    }
}

impl fmt::Display for PhoneNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhoneNumberError::Empty => write!(f, "電話番号が入力されていません"),
            PhoneNumberError::InvalidFormat => write!(f, "電話番号の形式が正しくありません"),
            PhoneNumberError::TooLong => write!(f, "電話番号が長すぎます"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_phone_numbers() {
        assert!(PhoneNumber::new("03-1234-5678".to_string()).is_ok());
        assert!(PhoneNumber::new("090-1234-5678".to_string()).is_ok());
        assert!(PhoneNumber::new("0312345678".to_string()).is_ok());
        assert!(PhoneNumber::new("09012345678".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_phone_numbers() {
        assert_eq!(
            PhoneNumber::new("".to_string()),
            Err(PhoneNumberError::Empty)
        );
        assert_eq!(
            PhoneNumber::new("123-456-789".to_string()),
            Err(PhoneNumberError::InvalidFormat)
        );
        assert_eq!(
            PhoneNumber::new("012345678901234567890".to_string()),
            Err(PhoneNumberError::TooLong)
        );
    }

    #[test]
    fn test_formatted_display() {
        let phone = PhoneNumber::new("0312345678".to_string()).unwrap();
        assert_eq!(phone.formatted(), "03-1234-5678");
        
        let phone = PhoneNumber::new("09012345678".to_string()).unwrap();
        assert_eq!(phone.formatted(), "090-1234-5678");
    }
}