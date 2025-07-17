use crate::application::dto::CategoryListDTO;
use crate::application::error::RepositoryError;

#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn find_all(&self) -> Result<CategoryListDTO, RepositoryError>;
}
