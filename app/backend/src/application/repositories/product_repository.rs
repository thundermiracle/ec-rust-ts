use crate::application::error::RepositoryError;
use crate::application::dto::{ProductListDTO, ProductDTO};
use crate::domain::models::ProductId;

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductDTO>, RepositoryError>;
    async fn find_all(&self) -> Result<ProductListDTO, RepositoryError>;
}
