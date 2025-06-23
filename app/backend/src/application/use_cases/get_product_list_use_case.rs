use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::dto::ProductListDTO;

pub struct GetProductListUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetProductListUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_all(&self) -> Result<ProductListDTO, ApplicationError> {
        print!("->> get_product_list_usecase");
        
        let product_list = self.product_repository.find_all().await?;

        Ok(product_list)
    }
}
