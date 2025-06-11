use crate::domain::models::Product;

/// Application層での商品クエリオブジェクト
/// シリアライゼーションの詳細は含まない
pub struct GetProductQuery {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl From<Product> for GetProductQuery {
    fn from(product: Product) -> GetProductQuery {
        GetProductQuery {
            id: product.id,
            name: product.name,
            price: product.price,
            description: product.description,
            quantity: product.quantity,
        }
    }
}