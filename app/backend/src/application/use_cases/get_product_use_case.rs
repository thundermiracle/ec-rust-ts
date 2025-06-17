use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::queries::{ProductQuery, ProductQueryMapper};

pub struct GetProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<ProductQuery, ApplicationError> {
        print!("->> get_product_usecase");
        
        let product = self.product_repository.find_by_id(id).await?
            .ok_or(ApplicationError::ProductNotFound(id))?;

        let product_query = ProductQueryMapper::from_domain(product)?;
        
        Ok(product_query)
    }
}
