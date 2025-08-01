use crate::application::dto::VariantSummaryDTO;
use crate::application::error::ApplicationError;
use crate::application::queries::models::FindVariantsQuery;
use crate::application::repositories::VariantRepository;
use std::sync::Arc;

pub struct FindVariantsHandler {
    repository: Arc<dyn VariantRepository + Send + Sync>,
}

impl FindVariantsHandler {
    pub fn new(repository: Arc<dyn VariantRepository + Send + Sync>) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        query: FindVariantsQuery,
    ) -> Result<Vec<VariantSummaryDTO>, ApplicationError> {
        if query.sku_ids.is_empty() {
            return Ok(Vec::new());
        }

        self.repository.find_by_ids(query.sku_ids).await
    }
}
