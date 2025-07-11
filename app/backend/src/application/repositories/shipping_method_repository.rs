use async_trait::async_trait;
use crate::application::dto::ShippingMethodListDTO;
use crate::application::error::RepositoryError;

#[async_trait]
pub trait ShippingMethodRepository: Send + Sync {
    /// アクティブな配送方法をすべて取得（sort_order順）
    async fn find_all(&self) -> Result<ShippingMethodListDTO, RepositoryError>;
}