use async_trait::async_trait;
use crate::domain::SKUId;
use crate::application::dto::VariantSummaryDTO;
use crate::application::error::ApplicationError;

#[async_trait]
pub trait VariantRepository {
    async fn find_by_ids(&self, ids: Vec<SKUId>) -> Result<Vec<VariantSummaryDTO>, ApplicationError>;
} 