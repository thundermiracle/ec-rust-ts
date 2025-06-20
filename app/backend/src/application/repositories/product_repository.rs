use crate::application::error::RepositoryError;
use crate::application::viewmodels::ProductViewModel;
use crate::domain::models::ProductId;

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductViewModel>, RepositoryError>;
}
