use uuid::Uuid;

use crate::domain::ProductId;


/// 商品クエリ結果 - API応答に最適化されたDTO
/// Clean Architecture: Application層のQuery DTO.
/// Presentation層に渡すための、ビジネスロジックを含まない純粋なデータ構造です。
#[derive(Debug, Clone)]
pub struct GetProductQuery {
    // 基本情報
    pub product_id: ProductId,
}

impl GetProductQuery {
    pub fn new(product_id: String) -> Self {
        Self {
            product_id: ProductId::from_uuid(Uuid::parse_str(&product_id).unwrap()),
        }
    }
}