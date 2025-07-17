use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FirstName {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LastName {
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonalInfo {
    first_name: FirstName,
    last_name: LastName,
}

#[derive(Debug, PartialEq)]
pub enum PersonalInfoError {
    Empty,
    TooLong,
    InvalidCharacter,
}

impl FirstName {
    pub fn new(value: String) -> Result<Self, PersonalInfoError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(value: &str) -> Result<(), PersonalInfoError> {
        if value.trim().is_empty() {
            return Err(PersonalInfoError::Empty);
        }

        if value.len() > 50 {
            return Err(PersonalInfoError::TooLong);
        }

        // 日本語、英語、数字、一般的な記号を許可
        if !value.chars().all(|c| {
            c.is_alphanumeric() || c.is_whitespace() || "ー・".contains(c) || (c as u32) >= 0x3040 // ひらがな・カタカナ・漢字
        }) {
            return Err(PersonalInfoError::InvalidCharacter);
        }

        Ok(())
    }
}

impl LastName {
    pub fn new(value: String) -> Result<Self, PersonalInfoError> {
        Self::validate(&value)?;
        Ok(Self { value })
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn validate(value: &str) -> Result<(), PersonalInfoError> {
        if value.trim().is_empty() {
            return Err(PersonalInfoError::Empty);
        }

        if value.len() > 50 {
            return Err(PersonalInfoError::TooLong);
        }

        // 日本語、英語、数字、一般的な記号を許可
        if !value.chars().all(|c| {
            c.is_alphanumeric() || c.is_whitespace() || "ー・".contains(c) || (c as u32) >= 0x3040 // ひらがな・カタカナ・漢字
        }) {
            return Err(PersonalInfoError::InvalidCharacter);
        }

        Ok(())
    }
}

impl PersonalInfo {
    pub fn new(first_name: FirstName, last_name: LastName) -> Self {
        Self {
            first_name,
            last_name,
        }
    }

    pub fn from_strings(first_name: String, last_name: String) -> Result<Self, PersonalInfoError> {
        let first_name = FirstName::new(first_name)?;
        let last_name = LastName::new(last_name)?;
        Ok(Self::new(first_name, last_name))
    }

    pub fn first_name(&self) -> &FirstName {
        &self.first_name
    }

    pub fn last_name(&self) -> &LastName {
        &self.last_name
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name.value(), self.last_name.value())
    }
}

impl fmt::Display for FirstName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for LastName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl fmt::Display for PersonalInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

impl fmt::Display for PersonalInfoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PersonalInfoError::Empty => write!(f, "名前が入力されていません"),
            PersonalInfoError::TooLong => write!(f, "名前が長すぎます（50文字以内）"),
            PersonalInfoError::InvalidCharacter => {
                write!(f, "名前に使用できない文字が含まれています")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_names() {
        assert!(FirstName::new("太郎".to_string()).is_ok());
        assert!(LastName::new("田中".to_string()).is_ok());
        assert!(FirstName::new("Taro".to_string()).is_ok());
        assert!(LastName::new("Tanaka".to_string()).is_ok());
        assert!(FirstName::new("太郎・次郎".to_string()).is_ok());
    }

    #[test]
    fn test_invalid_names() {
        assert_eq!(
            FirstName::new("".to_string()),
            Err(PersonalInfoError::Empty)
        );
        assert_eq!(
            FirstName::new("a".repeat(51)),
            Err(PersonalInfoError::TooLong)
        );
        assert_eq!(
            FirstName::new("太郎@".to_string()),
            Err(PersonalInfoError::InvalidCharacter)
        );
    }

    #[test]
    fn test_personal_info() {
        let personal_info =
            PersonalInfo::from_strings("太郎".to_string(), "田中".to_string()).unwrap();
        assert_eq!(personal_info.full_name(), "太郎 田中");
        assert_eq!(personal_info.first_name().value(), "太郎");
        assert_eq!(personal_info.last_name().value(), "田中");
    }
}
