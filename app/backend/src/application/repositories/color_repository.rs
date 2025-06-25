use crate::application::{dto::ColorListDTO, error::RepositoryError};

#[async_trait::async_trait]
pub trait ColorRepository {
    async fn find_all(&self) -> Result<ColorListDTO, RepositoryError>;
}
