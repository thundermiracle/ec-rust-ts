use async_trait::async_trait;
use crate::application::dto::ShippingMethodListDTO;
use crate::application::error::RepositoryError;
use crate::domain::entities::ShippingMethod;

#[async_trait]
pub trait ShippingMethodRepository: Send + Sync {
    /// アクティブな配送方法をすべて取得（sort_order順）
    async fn find_all(&self) -> Result<ShippingMethodListDTO, RepositoryError>;
    
    
    /// IDで配送方法エンティティを取得（ドメイン計算用）
    async fn find_by_id(&self, id: &str) -> Result<Option<ShippingMethod>, RepositoryError>;
}