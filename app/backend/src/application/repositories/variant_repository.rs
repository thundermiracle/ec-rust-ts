use async_trait::async_trait;
use crate::domain::models::SKUId;
use crate::application::dto::VariantInfoDTO;
use crate::application::error::ApplicationError;

#[async_trait]
pub trait VariantRepository {
    async fn find_by_ids(&self, ids: Vec<SKUId>) -> Result<Vec<VariantInfoDTO>, ApplicationError>;
} 