use crate::application::error::RepositoryError;
use crate::application::viewmodels::{ProductListViewModel, ProductViewModel};
use crate::domain::models::ProductId;

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductViewModel>, RepositoryError>;
    async fn find_all(&self) -> Result<ProductListViewModel, RepositoryError>;
}
