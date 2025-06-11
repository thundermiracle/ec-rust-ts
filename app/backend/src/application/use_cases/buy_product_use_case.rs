use std::sync::Arc;

use crate::application::repositories::ProductRepository;
use crate::application::error::ApplicationError;
use crate::application::commands::BuyProductCommand;

pub struct BuyProductUseCase {
    product_repository: Arc<dyn ProductRepository + Send + Sync>,
}

impl BuyProductUseCase {
    pub fn new(product_repository: Arc<dyn ProductRepository + Send + Sync>) -> Self {
        Self {
            product_repository,
        }
    }

    pub async fn buy(&self, product_id: u32, command: BuyProductCommand) -> Result<(), ApplicationError> {
        print!("->> buy_product_usecase");
        
        match self.product_repository.find_by_id(product_id).await? {
            Some(mut product) => {
                product.sell(command.quantity)?;
                self.product_repository.save(product).await?;
                Ok(())
            }
            None => Err(ApplicationError::ProductNotFound(product_id)),
        }
    }
}
