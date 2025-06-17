use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::queries::ProductSummary;

pub struct GetAllProductsUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl GetAllProductsUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<ProductSummary>, ApplicationError> {
        print!("->> get_all_products_usecase");
        let products = self.product_repository.find_all().await?;
        let result = products.into_iter().map(|p| p.into()).collect();
        Ok(result)
    }
}