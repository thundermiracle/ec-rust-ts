use crate::domain::error::DomainError;

/// 色ドメインモデル
/// 製品の色を管理し、表示名とHEXコード（オプション）を保持
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub name: ColorName,
    pub hex_code: Option<String>,
}

/// 色名値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColorName(String);

impl Color {
    /// 新しい色を作成
    pub fn new(name: ColorName, hex_code: Option<String>) -> Result<Self, DomainError> {
        // HEXコードのバリデーション
        if let Some(ref hex) = hex_code {
            Self::validate_hex_code(hex)?;
        }

        Ok(Self { name, hex_code })
    }

    /// HEXコードなしで色を作成
    pub fn from_name(name: ColorName) -> Self {
        Self {
            name,
            hex_code: None,
        }
    }

    /// HEXコード付きで色を作成
    pub fn with_hex(name: ColorName, hex_code: String) -> Result<Self, DomainError> {
        Self::validate_hex_code(&hex_code)?;
        Ok(Self {
            name,
            hex_code: Some(hex_code),
        })
    }

    /// HEXコードのバリデーション
    fn validate_hex_code(hex: &str) -> Result<(), DomainError> {
        // HEXコードは#で始まり、6桁の16進数であること
        if !hex.starts_with('#') || hex.len() != 7 {
            return Err(DomainError::InvalidProductData(
                "Hex code must start with # and be 7 characters long".to_string(),
            ));
        }

        // 16進数かどうかをチェック
        if !hex[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidProductData(
                "Hex code must contain only hexadecimal digits".to_string(),
            ));
        }

        Ok(())
    }

    /// 色の名前を取得
    pub fn name(&self) -> &ColorName {
        &self.name
    }

    /// HEXコードを取得
    pub fn hex_code(&self) -> Option<&str> {
        self.hex_code.as_deref()
    }
}

impl ColorName {
    /// 新しい色名を作成
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed = name.trim();
        
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Color name cannot be empty".to_string(),
            ));
        }

        // 色名は最大50文字まで
        if trimmed.len() > 50 {
            return Err(DomainError::InvalidProductData(
                "Color name cannot exceed 50 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// 色名の値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ColorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.hex_code {
            Some(hex) => write!(f, "{} ({})", self.name, hex),
            None => write!(f, "{}", self.name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_without_hex() {
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::from_name(color_name);
        assert_eq!(color.name().value(), "Walnut");
        assert!(color.hex_code().is_none());
    }

    #[test]
    fn create_color_with_valid_hex() {
        let color_name = ColorName::new("Black".to_string()).unwrap();
        let color = Color::with_hex(color_name, "#000000".to_string());
        assert!(color.is_ok());
        let color = color.unwrap();
        assert_eq!(color.name().value(), "Black");
        assert_eq!(color.hex_code(), Some("#000000"));
    }

    #[test]
    fn reject_invalid_hex_format() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::with_hex(color_name, "FF0000".to_string());
        assert!(color.is_err());
    }

    #[test]
    fn reject_invalid_hex_length() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::with_hex(color_name, "#FF00".to_string());
        assert!(color.is_err());
    }

    #[test]
    fn reject_invalid_hex_characters() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::with_hex(color_name, "#GGGGGG".to_string());
        assert!(color.is_err());
    }

    #[test]
    fn reject_empty_color_name() {
        let color_name = ColorName::new("".to_string());
        assert!(color_name.is_err());
    }

    #[test]
    fn reject_long_color_name() {
        let long_name = "a".repeat(51);
        let color_name = ColorName::new(long_name);
        assert!(color_name.is_err());
    }
} 