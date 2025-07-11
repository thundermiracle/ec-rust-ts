use std::sync::Arc;
use crate::application::repositories::PaymentMethodRepository;
use crate::application::error::ApplicationError;
use crate::application::dto::PaymentMethodListDTO;

/// PaymentMethodリスト取得クエリハンドラ
pub struct GetPaymentMethodListHandler {
    payment_method_repository: Arc<dyn PaymentMethodRepository + Send + Sync>,
}

impl GetPaymentMethodListHandler {
    pub fn new(payment_method_repository: Arc<dyn PaymentMethodRepository + Send + Sync>) -> Self {
        Self { payment_method_repository }
    }

    /// クエリを実行
    pub async fn handle(&self) -> Result<PaymentMethodListDTO, ApplicationError> {
        println!("->> GetPaymentMethodListHandler::handle");
        
        let result = self.payment_method_repository.find_all().await?;
        
        Ok(result)
    }
}