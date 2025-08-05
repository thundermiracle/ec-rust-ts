use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::application::repositories::CouponRepository;
use crate::domain::entities::Coupon;
use crate::domain::error::DomainError;
use crate::domain::value_objects::CouponCode;

/// SQLite実装のCouponRepository
/// Clean Architecture: Frameworks & Drivers層
/// 一時的なスタブ実装 - コンパイルを通すためのダミー実装
pub struct SqliteCouponRepository {
    #[allow(dead_code)]
    pool: SqlitePool,
}

impl SqliteCouponRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CouponRepository for SqliteCouponRepository {
    /// クーポンコードでクーポンを検索
    /// 現在はスタブ実装として常にNoneを返す
    async fn find_by_code(&self, code: &CouponCode) -> Result<Option<Coupon>, DomainError> {
        // TODO: 実際のデータベース実装に置き換える
        // 現在は常にクーポンが見つからないことを返す
        let _ = code; // unused parameter warning を避ける
        Ok(None)
    }

    /// クーポンの使用回数を更新
    /// 現在はスタブ実装として常に成功を返す
    async fn update_usage_count(&self, coupon: &Coupon) -> Result<(), DomainError> {
        // TODO: 実際のデータベース実装に置き換える
        // 現在は何も行わずに成功を返す
        let _ = coupon; // unused parameter warning を避ける
        Ok(())
    }
}