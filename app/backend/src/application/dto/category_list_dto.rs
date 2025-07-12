#[derive(Debug, Clone)]
pub struct CategoryListDTO {
    pub categories: Vec<CategoryDTO>,
}

#[derive(Debug, Clone)]
pub struct CategoryDTO {
    pub id: String,
    pub name: String,
    pub slug: String,
    pub parent_id: Option<String>,
    pub display_order: u32,
}

impl CategoryListDTO {
    pub fn new(categories: Vec<CategoryDTO>) -> Self {
        Self { categories }
    }
}

impl CategoryDTO {
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
    fn test_category_dto_creation() {
        let category = CategoryDTO::new(
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
    fn test_subcategory_dto_creation() {
        let category = CategoryDTO::new(
            "office-desks".to_string(),
            "Office Desks".to_string(),
            "office-desks".to_string(),
            Some("desks".to_string()),
            1,
        );

        assert_eq!(category.parent_id, Some("desks".to_string()));
        assert!(!category.is_root());
        assert!(category.is_subcategory());
    }

    #[test]
    fn test_category_list_dto_creation() {
        let categories = vec![
            CategoryDTO::new("desks".to_string(), "Desks".to_string(), "desks".to_string(), None, 1),
            CategoryDTO::new("tables".to_string(), "Tables".to_string(), "tables".to_string(), None, 2),
        ];

        let category_list = CategoryListDTO::new(categories);
        assert_eq!(category_list.categories.len(), 2);
    }
} 