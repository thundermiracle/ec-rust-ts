use crate::application::dto::{CategoryListDTO, CategoryDTO};
use crate::presentation::categories::responses::{CategoryListResponse, CategoryResponse};

/// カテゴリリストプレゼンター
/// Clean Architecture: Interface Adapters層
/// アプリケーション層のDTOをHTTPレスポンス用DTOに変換する
pub struct CategoryListPresenter;

impl CategoryListPresenter {
    /// CategoryListDTOをCategoryListResponseに変換
    ///
    /// # Arguments
    /// * `dto` - アプリケーション層から取得したCategoryListDTO
    ///
    /// # Returns
    /// HTTPレスポンス用のCategoryListResponse
    ///
    /// # Example
    /// ```rust
    /// let response = CategoryListPresenter::present(category_list_dto);
    /// ```
    pub fn present(dto: CategoryListDTO) -> CategoryListResponse {
        let categories = dto
            .categories
            .into_iter()
            .map(Self::present_category_item)
            .collect();

        CategoryListResponse::new(categories)
    }

    /// CategoryDTOをCategoryResponseに変換
    ///
    /// # Arguments
    /// * `dto` - カテゴリのDTO
    ///
    /// # Returns
    /// HTTPレスポンス用のCategoryResponse
    fn present_category_item(dto: CategoryDTO) -> CategoryResponse {
        CategoryResponse::new(
            dto.id,
            dto.name,
            dto.slug,
            dto.parent_id,
            dto.display_order,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{CategoryListDTO, CategoryDTO};

    /// CategoryListPresenterのテスト
    #[test]
    fn test_present_category_list() {
        // Given: CategoryListDTOの作成
        let categories = vec![
            CategoryDTO::new(
                "desks".to_string(),
                "Desks".to_string(),
                "desks".to_string(),
                None,
                1,
            ),
            CategoryDTO::new(
                "tables".to_string(),
                "Tables".to_string(),
                "tables".to_string(),
                None,
                2,
            ),
        ];
        let category_list_dto = CategoryListDTO::new(categories);

        // When: CategoryListPresenterで変換
        let response = CategoryListPresenter::present(category_list_dto);

        // Then: 正しく変換されている
        assert_eq!(response.categories.len(), 2);
        assert_eq!(response.categories[0].id, "desks");
        assert_eq!(response.categories[0].name, "Desks");
        assert_eq!(response.categories[0].slug, "desks");
        assert_eq!(response.categories[0].display_order, 1);
        assert!(response.categories[0].is_root());

        assert_eq!(response.categories[1].id, "tables");
        assert_eq!(response.categories[1].name, "Tables");
        assert_eq!(response.categories[1].slug, "tables");
        assert_eq!(response.categories[1].display_order, 2);
        assert!(response.categories[1].is_root());
    }

    #[test]
    fn test_present_subcategory() {
        // Given: サブカテゴリを含むCategoryListDTOの作成
        let categories = vec![
            CategoryDTO::new(
                "furniture".to_string(),
                "Furniture".to_string(),
                "furniture".to_string(),
                None,
                1,
            ),
            CategoryDTO::new(
                "office-desks".to_string(),
                "Office Desks".to_string(),
                "office-desks".to_string(),
                Some("furniture".to_string()),
                1,
            ),
        ];
        let category_list_dto = CategoryListDTO::new(categories);

        // When: CategoryListPresenterで変換
        let response = CategoryListPresenter::present(category_list_dto);

        // Then: 階層構造が正しく変換されている
        assert_eq!(response.categories.len(), 2);
        assert!(response.categories[0].is_root());
        assert!(response.categories[1].is_subcategory());
        assert_eq!(response.categories[1].parent_id, Some("furniture".to_string()));
    }
} 