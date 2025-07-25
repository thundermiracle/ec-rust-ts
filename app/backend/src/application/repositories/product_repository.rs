use crate::application::dto::{ProductDTO, ProductListDTO, VariantDTO};
use crate::application::error::RepositoryError;
use crate::domain::{ProductId, SKUId};

#[async_trait::async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductDTO>, RepositoryError>;
    async fn find_all(&self) -> Result<ProductListDTO, RepositoryError>;

    /// 複数のSKU IDでバリアント（SKU）を取得
    async fn find_variants_by_ids(
        &self,
        sku_ids: &[SKUId],
    ) -> Result<Vec<VariantDTO>, RepositoryError>;
}
