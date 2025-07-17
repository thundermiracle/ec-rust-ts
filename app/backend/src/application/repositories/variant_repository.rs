use crate::application::dto::VariantSummaryDTO;
use crate::application::error::ApplicationError;
use crate::domain::SKUId;
use async_trait::async_trait;

#[async_trait]
pub trait VariantRepository: Send + Sync {
    async fn find_by_ids(
        &self,
        ids: Vec<SKUId>,
    ) -> Result<Vec<VariantSummaryDTO>, ApplicationError>;
}
