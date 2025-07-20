use crate::application::error::RepositoryError;
use crate::domain::aggregates::order::Order;

#[async_trait::async_trait]
pub trait OrderRepository: Send + Sync {
    /// 注文を保存
    async fn save(&self, order: &Order) -> Result<(), RepositoryError>;

    /// 注文を更新
    async fn update(&self, order: &Order) -> Result<(), RepositoryError>;
    
    /// 指定された年の次のシーケンス番号を取得
    async fn get_next_sequence_number(&self, year: i32) -> Result<u32, RepositoryError>;
}
