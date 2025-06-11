use serde::{Deserialize, Serialize};
use crate::application::queries::GetProductQuery;

/// Product Presenter - レスポンス形式の整形を担当
/// Uncle Bob's Clean Architecture における Presenter の役割
#[derive(Serialize, Deserialize)]
pub struct ProductPresenter {
    pub id: u32,
    pub name: String,
    pub price: u32,
    pub description: String,
    pub quantity: u32,
}

/// Application層のQueryからPresenterへの変換
/// この変換がPresenterの主要な責任
impl From<GetProductQuery> for ProductPresenter {
    fn from(query: GetProductQuery) -> Self {
        ProductPresenter {
            id: query.id,
            name: query.name,
            price: query.price,
            description: query.description,
            quantity: query.quantity,
        }
    }
}

impl ProductPresenter {
    /// 必要に応じてフォーマット処理を追加
    /// 例: 価格の通貨表示、在庫状況の文字列化など
    pub fn format_price_display(&self) -> String {
        format!("¥{}", self.price)
    }

    pub fn format_availability_status(&self) -> String {
        if self.quantity > 0 {
            "在庫あり".to_string()
        } else {
            "在庫切れ".to_string()
        }
    }
} 