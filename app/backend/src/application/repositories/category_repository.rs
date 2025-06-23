use crate::application::error::RepositoryError;
use crate::application::dto::CategoryListDTO;

#[async_trait::async_trait]
pub trait CategoryRepository {
    async fn find_all(&self) -> Result<CategoryListDTO, RepositoryError>;
} 