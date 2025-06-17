use crate::domain::models::Product;

/// Application層での商品概要オブジェクト
/// ドメインモデルから必要な情報のみを抽出したシンプルなデータ構造
pub struct ProductSummary {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

impl From<Product> for ProductSummary {
    fn from(product: Product) -> ProductSummary {
        ProductSummary {
            id: product.id,
            name: product.name.clone(),
            price: product.current_price().yen(),
            description: product.description.clone(),
            quantity: product.quantity,
        }
    }
}