use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::queries::get_product_query::GetProductQuery;
use crate::application::dto::ProductDTO;

pub struct GetProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_by_id(&self, get_product_query: GetProductQuery) -> Result<ProductDTO, ApplicationError> {
        print!("->> get_product_usecase");
        
        let product = self.product_repository.find_by_id(&get_product_query.product_id).await?
            .ok_or(ApplicationError::ProductNotFound(get_product_query.product_id.to_string()))?;

        
        Ok(product)
    }
}
