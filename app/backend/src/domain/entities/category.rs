use crate::domain::error::DomainError;

/// カテゴリドメインモデル
/// 階層構造をサポートし、親子関係を管理
#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub id: CategoryId,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<CategoryId>,
    pub display_order: u32,
}

/// カテゴリID値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CategoryId(String);

impl Category {
    /// 新しいカテゴリを作成
    pub fn new(
        id: CategoryId,
        name: String,
        slug: String,
        parent_id: Option<CategoryId>,
        display_order: Option<u32>,
    ) -> Result<Self, DomainError> {
        // ビジネスルール: 名前とスラッグは空文字列不可
        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Category name cannot be empty".to_string(),
            ));
        }

        if slug.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Category slug cannot be empty".to_string(),
            ));
        }

        // ビジネスルール: スラッグは英数字とハイフンのみ
        if !slug.chars().all(|c| c.is_alphanumeric() || c == '-') {
            return Err(DomainError::InvalidProductData(
                "Category slug must contain only alphanumeric characters and hyphens".to_string(),
            ));
        }

        Ok(Self {
            id,
            name,
            slug,
            parent_id,
            display_order: display_order.unwrap_or(0),
        })
    }

    /// ルートカテゴリかどうかを判定
    pub fn is_root(&self) -> bool {
        self.parent_id.is_none()
    }

    /// サブカテゴリかどうかを判定
    pub fn is_subcategory(&self) -> bool {
        self.parent_id.is_some()
    }

    /// 親カテゴリIDを取得
    pub fn parent_id(&self) -> Option<&CategoryId> {
        self.parent_id.as_ref()
    }
}

impl CategoryId {
    /// 新しいカテゴリIDを作成
    pub fn new(id: String) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Category ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(id))
    }

    /// IDの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for CategoryId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Category {
    fn default() -> Self {
        Self {
            id: CategoryId("default".to_string()),
            name: "Default Category".to_string(),
            slug: "default".to_string(),
            parent_id: None,
            display_order: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_category() {
        let category_id = CategoryId::new("desks".to_string()).unwrap();
        let category = Category::new(
            category_id.clone(),
            "Desks".to_string(),
            "desks".to_string(),
            None,
            None,
        );
        assert!(category.is_ok());
        let category = category.unwrap();
        assert!(category.is_root());
        assert!(!category.is_subcategory());
    }

    #[test]
    fn create_subcategory() {
        let parent_id = CategoryId::new("furniture".to_string()).unwrap();
        let category_id = CategoryId::new("desks".to_string()).unwrap();
        let category = Category::new(
            category_id,
            "Desks".to_string(),
            "desks".to_string(),
            Some(parent_id),
            None,
        );
        assert!(category.is_ok());
        let category = category.unwrap();
        assert!(!category.is_root());
        assert!(category.is_subcategory());
    }

    #[test]
    fn reject_empty_name() {
        let category_id = CategoryId::new("desks".to_string()).unwrap();
        let category = Category::new(category_id, "".to_string(), "desks".to_string(), None, None);
        assert!(category.is_err());
    }

    #[test]
    fn reject_invalid_slug() {
        let category_id = CategoryId::new("desks".to_string()).unwrap();
        let category = Category::new(
            category_id,
            "Desks".to_string(),
            "desks with spaces".to_string(),
            None,
            None,
        );
        assert!(category.is_err());
    }
}
