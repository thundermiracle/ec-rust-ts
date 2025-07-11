use crate::application::error::RepositoryError;
use crate::application::dto::PaymentMethodListDTO;

#[async_trait::async_trait]
pub trait PaymentMethodRepository {
    async fn find_all(&self) -> Result<PaymentMethodListDTO, RepositoryError>;
}