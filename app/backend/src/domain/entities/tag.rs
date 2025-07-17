use crate::domain::error::DomainError;

/// タグドメインモデル  
/// 商品の動的フラグ管理（best_seller, quick_ship等）を担当
#[derive(Debug, Clone, PartialEq)]
pub struct Tag {
    pub slug: TagSlug,
    pub name: String,
    pub color_code: Option<String>,
    pub priority: u8,
    pub is_system: bool,
}

/// タグスラッグ値オブジェクト（データベースキー用）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TagSlug(String);

impl Tag {
    /// 新しいタグを作成
    pub fn new(
        slug: TagSlug,
        name: String,
        color_code: Option<String>,
        priority: u8,
        is_system: bool,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 表示名は空不可
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Tag name cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: 色指定がある場合はHEX形式かチェック
        if let Some(ref color) = color_code {
            Self::validate_color_code(color)?;
        }

        Ok(Self {
            slug,
            name: name.trim().to_string(),
            color_code,
            priority,
            is_system,
        })
    }

    /// よく使われるシステムタグを事前定義
    pub fn on_sale() -> Result<Self, DomainError> {
        Self::new(
            TagSlug::new("on_sale".to_string())?,
            "On Sale".to_string(),
            Some("#FF6B6B".to_string()),
            1, // 最高優先度
            true,
        )
    }

    pub fn best_seller() -> Result<Self, DomainError> {
        Self::new(
            TagSlug::new("best_seller".to_string())?,
            "Best Seller".to_string(),
            Some("#4ECDC4".to_string()),
            2,
            true,
        )
    }

    pub fn quick_ship() -> Result<Self, DomainError> {
        Self::new(
            TagSlug::new("quick_ship".to_string())?,
            "Quick Ship".to_string(),
            Some("#45B7D1".to_string()),
            3,
            true,
        )
    }

    pub fn new_arrival() -> Result<Self, DomainError> {
        Self::new(
            TagSlug::new("new_arrival".to_string())?,
            "New Arrival".to_string(),
            Some("#96CEB4".to_string()),
            4,
            true,
        )
    }

    pub fn sold_out() -> Result<Self, DomainError> {
        Self::new(
            TagSlug::new("sold_out".to_string())?,
            "Sold Out".to_string(),
            Some("#FFEAA7".to_string()),
            5,
            true,
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

    /// タグスラッグを取得
    pub fn slug(&self) -> &TagSlug {
        &self.slug
    }

    /// タグ名を取得
    pub fn name(&self) -> &str {
        &self.name
    }

    /// 色コードを取得
    pub fn color_code(&self) -> Option<&str> {
        self.color_code.as_deref()
    }

    /// 優先度を取得
    pub fn priority(&self) -> u8 {
        self.priority
    }

    /// システムタグかどうかを判定
    pub fn is_system_tag(&self) -> bool {
        self.is_system
    }

    /// 高優先度タグかどうかを判定（優先度1-3）
    pub fn is_high_priority(&self) -> bool {
        self.priority <= 3
    }
}

impl TagSlug {
    /// 新しいタグスラッグを作成
    pub fn new(slug: String) -> Result<Self, DomainError> {
        let trimmed = slug.trim();

        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Tag slug cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: スラッグは小文字英数字とアンダースコアのみ
        if !trimmed
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            return Err(DomainError::InvalidProductData(
                "Tag slug must contain only lowercase letters, digits, and underscores".to_string(),
            ));
        }

        // ビジネスルール: スラッグは最大50文字
        if trimmed.len() > 50 {
            return Err(DomainError::InvalidProductData(
                "Tag slug cannot exceed 50 characters".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// スラッグの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for TagSlug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.slug)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_tag() {
        let tag_slug = TagSlug::new("custom_tag".to_string()).unwrap();
        let tag = Tag::new(
            tag_slug,
            "Custom Tag".to_string(),
            Some("#FF0000".to_string()),
            10,
            false,
        );

        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.name(), "Custom Tag");
        assert_eq!(tag.slug().value(), "custom_tag");
        assert!(!tag.is_high_priority());
        assert!(!tag.is_system_tag());
    }

    #[test]
    fn create_predefined_system_tags() {
        let on_sale = Tag::on_sale();
        assert!(on_sale.is_ok());
        let on_sale = on_sale.unwrap();
        assert!(on_sale.is_high_priority());
        assert!(on_sale.is_system_tag());
        assert_eq!(on_sale.slug().value(), "on_sale");
        assert_eq!(on_sale.name(), "On Sale");

        let best_seller = Tag::best_seller();
        assert!(best_seller.is_ok());
        let best_seller = best_seller.unwrap();
        assert!(best_seller.is_high_priority());
        assert!(best_seller.is_system_tag());
        assert_eq!(best_seller.slug().value(), "best_seller");
    }

    #[test]
    fn reject_invalid_tag_slug() {
        // 大文字を含む
        assert!(TagSlug::new("Invalid_Tag".to_string()).is_err());

        // 空文字
        assert!(TagSlug::new("".to_string()).is_err());

        // 特殊文字を含む
        assert!(TagSlug::new("tag-name".to_string()).is_err());
        assert!(TagSlug::new("tag name".to_string()).is_err());
    }

    #[test]
    fn reject_long_tag_slug() {
        let long_slug = "a".repeat(51);
        assert!(TagSlug::new(long_slug).is_err());
    }

    #[test]
    fn reject_empty_tag_name() {
        let tag_slug = TagSlug::new("test_tag".to_string()).unwrap();
        let tag = Tag::new(tag_slug, "".to_string(), None, 10, false);
        assert!(tag.is_err());
    }

    #[test]
    fn reject_invalid_color_code() {
        let tag_slug = TagSlug::new("test_tag".to_string()).unwrap();

        // HEX形式でない
        let tag = Tag::new(
            tag_slug.clone(),
            "Test Tag".to_string(),
            Some("FF0000".to_string()),
            10,
            false,
        );
        assert!(tag.is_err());

        // 長さが不正
        let tag = Tag::new(
            tag_slug.clone(),
            "Test Tag".to_string(),
            Some("#FF00".to_string()),
            10,
            false,
        );
        assert!(tag.is_err());

        // 無効な文字を含む
        let tag = Tag::new(
            tag_slug,
            "Test Tag".to_string(),
            Some("#GGGGGG".to_string()),
            10,
            false,
        );
        assert!(tag.is_err());
    }

    #[test]
    fn valid_color_code() {
        let tag_slug = TagSlug::new("test_tag".to_string()).unwrap();
        let tag = Tag::new(
            tag_slug,
            "Test Tag".to_string(),
            Some("#FF6B6B".to_string()),
            10,
            false,
        );
        assert!(tag.is_ok());
        let tag = tag.unwrap();
        assert_eq!(tag.color_code(), Some("#FF6B6B"));
    }
}
