use crate::domain::error::DomainError;

/// タグドメインモデル  
/// 商品の動的フラグ管理（best_seller, quick_ship等）を担当
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub name: TagName,
    pub display_name: String,
    pub color: Option<String>,
    pub priority: u8,
}

/// タグ名値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagName(String);

impl Tag {
    /// 新しいタグを作成
    pub fn new(
        name: TagName,
        display_name: String,
        color: Option<String>,
        priority: u8,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 表示名は空不可
        if display_name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Tag display name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 色指定がある場合はHEX形式かチェック
        if let Some(ref color_code) = color {
            Self::validate_color_code(color_code)?;
        }

        Ok(Self {
            name,
            display_name: display_name.trim().to_string(),
            color,
            priority,
        })
    }

    /// よく使われるタグを事前定義
    pub fn best_seller() -> Result<Self, DomainError> {
        Self::new(
            TagName::new("best_seller".to_string())?,
            "Best Seller".to_string(),
            Some("#FF6B35".to_string()), // オレンジ色
            100, // 高優先度
        )
    }

    pub fn quick_ship() -> Result<Self, DomainError> {
        Self::new(
            TagName::new("quick_ship".to_string())?,
            "Quick Ship".to_string(),
            Some("#00C851".to_string()), // 緑色 
            80,
        )
    }

    pub fn on_sale() -> Result<Self, DomainError> {
        Self::new(
            TagName::new("on_sale".to_string())?,
            "On Sale".to_string(),
            Some("#FF4444".to_string()), // 赤色
            90,
        )
    }

    pub fn new_arrival() -> Result<Self, DomainError> {
        Self::new(
            TagName::new("new_arrival".to_string())?,
            "New Arrival".to_string(),
            Some("#007BFF".to_string()), // 青色
            70,
        )
    }

    pub fn sold_out() -> Result<Self, DomainError> {
        Self::new(
            TagName::new("sold_out".to_string())?,
            "Sold Out".to_string(),
            Some("#6C757D".to_string()), // グレー色
            60,
        )
    }

    /// 色コードのバリデーション（HEX形式）
    fn validate_color_code(color: &str) -> Result<(), DomainError> {
        if !color.starts_with('#') || color.len() != 7 {
            return Err(DomainError::InvalidProductData(
                "Color code must be in HEX format (#RRGGBB)".to_string(),
            ));
        }

        if !color[1..].chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(DomainError::InvalidProductData(
                "Color code must contain only hexadecimal digits".to_string(),
            ));
        }

        Ok(())
    }

    /// タグ名を取得
    pub fn name(&self) -> &TagName {
        &self.name
    }

    /// 高優先度タグかどうかを判定
    pub fn is_high_priority(&self) -> bool {
        self.priority >= 80
    }
}

impl TagName {
    /// 新しいタグ名を作成
    pub fn new(name: String) -> Result<Self, DomainError> {
        let trimmed = name.trim();
        
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Tag name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: タグ名は小文字英数字とアンダースコアのみ
        if !trimmed.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
            return Err(DomainError::InvalidProductData(
                "Tag name must contain only lowercase letters, digits, and underscores".to_string(),
            ));
        }

        // ビジネスルール: タグ名は最大30文字
        if trimmed.len() > 30 {
            return Err(DomainError::InvalidProductData(
                "Tag name cannot exceed 30 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// タグ名の値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TagName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (priority: {})", self.display_name, self.priority)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_tag() {
        let tag_name = TagName::new("custom_tag".to_string()).unwrap();
        let tag = Tag::new(
            tag_name,
            "Custom Tag".to_string(),
            Some("#FF0000".to_string()),
            50,
        );
        
        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.display_name, "Custom Tag");
        assert!(!tag.is_high_priority());
    }

    #[test]
    fn create_predefined_tags() {
        let best_seller = Tag::best_seller();
        assert!(best_seller.is_ok());
        let best_seller = best_seller.unwrap();
        assert!(best_seller.is_high_priority());
        assert_eq!(best_seller.name().value(), "best_seller");

        let quick_ship = Tag::quick_ship();
        assert!(quick_ship.is_ok());
        let quick_ship = quick_ship.unwrap();
        assert!(quick_ship.is_high_priority());
    }

    #[test]
    fn reject_invalid_tag_name() {
        // 大文字を含む
        assert!(TagName::new("Best_Seller".to_string()).is_err());
        
        // スペースを含む
        assert!(TagName::new("best seller".to_string()).is_err());
        
        // 特殊文字を含む
        assert!(TagName::new("best-seller".to_string()).is_err());
        
        // 空文字列
        assert!(TagName::new("".to_string()).is_err());
    }

    #[test]
    fn reject_long_tag_name() {
        let long_name = "a".repeat(31);
        assert!(TagName::new(long_name).is_err());
    }

    #[test]
    fn reject_empty_display_name() {
        let tag_name = TagName::new("test_tag".to_string()).unwrap();
        let tag = Tag::new(
            tag_name,
            "".to_string(),
            None,
            50,
        );
        assert!(tag.is_err());
    }

    #[test]
    fn reject_invalid_color_code() {
        let tag_name = TagName::new("test_tag".to_string()).unwrap();
        
        // HEX形式でない
        let tag = Tag::new(
            tag_name.clone(),
            "Test Tag".to_string(),
            Some("FF0000".to_string()),
            50,
        );
        assert!(tag.is_err());

        // 長さが違う
        let tag = Tag::new(
            tag_name.clone(),
            "Test Tag".to_string(),
            Some("#FF00".to_string()),
            50,
        );
        assert!(tag.is_err());

        // 無効な16進数
        let tag = Tag::new(
            tag_name,
            "Test Tag".to_string(),
            Some("#GGGGGG".to_string()),
            50,
        );
        assert!(tag.is_err());
    }
} 