use crate::application::error::RepositoryError;
use crate::domain::models::{Product, ProductId};

#[async_trait::async_trait]
pub trait ProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError>;
}
