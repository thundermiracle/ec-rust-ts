use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::queries::ProductDetail;

pub struct GetProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_by_id(&self, id: u32) -> Result<ProductDetail, ApplicationError> {
        print!("->> get_product_usecase");
        
        match self.product_repository.find_by_id(id).await? {
            Some(product) => Ok(product.into()),
            None => Err(ApplicationError::ProductNotFound(id)),
        }
    }
}
