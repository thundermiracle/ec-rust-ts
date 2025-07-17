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
