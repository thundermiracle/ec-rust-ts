use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// カテゴリリストのHTTPレスポンス用DTO
/// Clean Architecture: Interface Adapters層
/// TypeScriptのCategory型と整合性を取った構造
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetCategoryListResponse {
    /// カテゴリ一覧
    pub categories: Vec<CategoryResponse>,
}

/// カテゴリのHTTPレスポンス用DTO
/// TypeScriptのCategory型に対応
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryResponse {
    /// カテゴリID
    pub id: String,
    /// カテゴリ名
    pub name: String,
    /// カテゴリスラッグ
    pub slug: String,
    /// 親カテゴリID（階層構造の場合）
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(nullable = false)]
    pub parent_id: Option<String>,
    /// 表示順序
    pub display_order: u32,
}

impl GetCategoryListResponse {
    pub fn new(categories: Vec<CategoryResponse>) -> Self {
        Self { categories }
    }
}

impl CategoryResponse {
    pub fn new(
        id: String,
        name: String,
        slug: String,
        parent_id: Option<String>,
        display_order: u32,
    ) -> Self {
        Self {
            id,
            name,
            slug,
            parent_id,
            display_order,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_response_creation() {
        let category = CategoryResponse::new(
            "desks".to_string(),
            "Desks".to_string(),
            "desks".to_string(),
            None,
            1,
        );

        assert_eq!(category.id, "desks");
        assert_eq!(category.name, "Desks");
        assert_eq!(category.slug, "desks");
        assert!(category.is_root());
        assert!(!category.is_subcategory());
    }

    #[test]
    fn test_get_category_list_response_creation() {
        let categories = vec![
            CategoryResponse::new(
                "desks".to_string(),
                "Desks".to_string(),
                "desks".to_string(),
                None,
                1,
            ),
            CategoryResponse::new(
                "tables".to_string(),
                "Tables".to_string(),
                "tables".to_string(),
                None,
                2,
            ),
        ];

        let category_list = GetCategoryListResponse::new(categories);
        assert_eq!(category_list.categories.len(), 2);
    }
}
