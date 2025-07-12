use crate::application::error::RepositoryError;
use crate::application::dto::PaymentMethodListDTO;
use crate::domain::entities::PaymentMethod;

#[async_trait::async_trait]
pub trait PaymentMethodRepository: Send + Sync {
    async fn find_all(&self) -> Result<PaymentMethodListDTO, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<Option<PaymentMethod>, RepositoryError>;
}