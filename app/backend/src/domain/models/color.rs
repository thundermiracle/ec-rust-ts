use crate::domain::error::DomainError;

/// 色ドメインモデル
/// 製品の色を管理し、表示名とHEXコードを保持
/// 新しいデータベーススキーマの集中型colorsテーブルに対応
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    /// 色のID（データベースからの主キー）
    pub id: u32,
    pub name: ColorName,
    pub hex: String,
}

/// 色名値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ColorName(String);

impl Color {
    /// 新しい色を作成
    pub fn new(
        id: u32,
        name: ColorName, 
        hex: String, 
    ) -> Result<Self, DomainError> {
        // HEXコードのバリデーション
        Self::validate_hex_code(&hex)?;

        Ok(Self { 
            id,
            name, 
            hex,
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
    pub fn hex_code(&self) -> &str {
        &self.hex
    }

    /// 色のIDを取得
    pub fn id(&self) -> u32 {
        self.id
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
        write!(f, "{} ({})", self.name, self.hex)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_color_with_valid_hex() {
        let color_name = ColorName::new("Walnut".to_string()).unwrap();
        let color = Color::new(1, color_name, "#8B4513".to_string());
        assert!(color.is_ok());
        let color = color.unwrap();
        assert_eq!(color.name().value(), "Walnut");
        assert_eq!(color.hex_code(), "#8B4513");
        assert_eq!(color.id(), 1);
    }

    #[test]
    fn create_color_without_id() {
        let color_name = ColorName::new("White Oak".to_string()).unwrap();
        let color = Color::new(2, color_name, "#F5F5DC".to_string());
        assert!(color.is_ok());
        let color = color.unwrap();
        assert_eq!(color.name().value(), "White Oak");
        assert_eq!(color.hex_code(), "#F5F5DC");
        assert_eq!(color.id(), 2);
    }

    #[test]
    fn reject_invalid_hex_format() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::new(3, color_name, "FF0000".to_string());
        assert!(color.is_err());
    }

    #[test]
    fn reject_invalid_hex_length() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::new(4, color_name, "#FF00".to_string());
        assert!(color.is_err());
    }

    #[test]
    fn reject_invalid_hex_characters() {
        let color_name = ColorName::new("Red".to_string()).unwrap();
        let color = Color::new(5, color_name, "#GGGGGG".to_string());
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