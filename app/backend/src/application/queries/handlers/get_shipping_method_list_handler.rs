use std::sync::Arc;

use crate::application::dto::ShippingMethodListDTO;
use crate::application::error::ApplicationError;
use crate::application::repositories::ShippingMethodRepository;

pub struct GetShippingMethodListHandler {
    shipping_method_repository: Arc<dyn ShippingMethodRepository + Send + Sync>,
}

impl GetShippingMethodListHandler {
    pub fn new(
        shipping_method_repository: Arc<dyn ShippingMethodRepository + Send + Sync>,
    ) -> Self {
        Self {
            shipping_method_repository,
        }
    }

    pub async fn handle(&self) -> Result<ShippingMethodListDTO, ApplicationError> {
        println!("->> GetShippingMethodListHandler::handle");

        let result = self.shipping_method_repository.find_all().await?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::dto::{ShippingMethodDTO, ShippingMethodListDTO};
    use crate::application::error::RepositoryError;
    use crate::application::repositories::ShippingMethodRepository;
    use crate::domain::entities::ShippingMethod;
    use async_trait::async_trait;

    // モックリポジトリ
    struct MockShippingMethodRepository {
        methods: ShippingMethodListDTO,
    }

    #[async_trait]
    impl ShippingMethodRepository for MockShippingMethodRepository {
        async fn find_all(&self) -> Result<ShippingMethodListDTO, RepositoryError> {
            Ok(self.methods.clone())
        }

        async fn find_by_id(&self, id: &str) -> Result<Option<ShippingMethod>, RepositoryError> {
            // テスト用の簡易実装
            // 実際の実装では適切なドメインエンティティを返す
            use crate::domain::value_objects::{Money, ShippingMethodId};

            let method = self.methods.methods.iter().find(|m| m.id == id);

            match method {
                Some(dto) => {
                    let shipping_method_id = ShippingMethodId::new(dto.id.clone())
                        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

                    let shipping_method = ShippingMethod::new(
                        shipping_method_id,
                        dto.name.clone(),
                        dto.description.clone(),
                        Money::from_yen(dto.price),
                        true,
                        1,
                    );
                    Ok(Some(shipping_method))
                }
                None => Ok(None),
            }
        }
    }

    #[tokio::test]
    async fn test_get_shipping_method_list_handler() {
        // テストデータ作成
        let methods = ShippingMethodListDTO::new(vec![
            ShippingMethodDTO {
                id: "standard".to_string(),
                name: "標準配送".to_string(),
                description: "5-7営業日".to_string(),
                price: 500,
            },
            ShippingMethodDTO {
                id: "express".to_string(),
                name: "速達配送".to_string(),
                description: "2-3営業日".to_string(),
                price: 1000,
            },
        ]);

        let repository = Arc::new(MockShippingMethodRepository { methods });
        let handler = GetShippingMethodListHandler::new(repository);

        // テスト実行
        let result = handler.handle().await.unwrap();

        // 検証
        assert_eq!(result.methods.len(), 2);
        assert_eq!(result.methods[0].id, "standard");
        assert_eq!(result.methods[0].name, "標準配送");
        assert_eq!(result.methods[0].price, 500);
        assert_eq!(result.methods[1].id, "express");
        assert_eq!(result.methods[1].name, "速達配送");
        assert_eq!(result.methods[1].price, 1000);
    }
}
