use crate::domain::entities::Coupon;
use crate::domain::error::DomainError;
use crate::domain::value_objects::CouponCode;
use async_trait::async_trait;

/// クーポンリポジトリトレイト
#[async_trait]
pub trait CouponRepository: Send + Sync {
    /// クーポンコードでクーポンを検索
    async fn find_by_code(&self, code: &CouponCode) -> Result<Option<Coupon>, DomainError>;

    /// クーポンの使用回数を更新
    async fn update_usage_count(&self, coupon: &Coupon) -> Result<(), DomainError>;
}
