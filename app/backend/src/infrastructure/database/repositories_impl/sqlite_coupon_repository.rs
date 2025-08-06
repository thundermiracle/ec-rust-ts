use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

use crate::application::repositories::CouponRepository;
use crate::domain::entities::Coupon;
use crate::domain::error::DomainError;
use crate::domain::value_objects::{CouponCode, CouponId, DiscountCondition, DiscountPolicy, DiscountType, Money};

/// SQLite実装のCouponRepository
/// Clean Architecture: Frameworks & Drivers層
pub struct SqliteCouponRepository {
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
    async fn find_by_code(&self, code: &CouponCode) -> Result<Option<Coupon>, DomainError> {
        let code_str = code.value();
        
        let row = sqlx::query(
            r#"
            SELECT id, code, name, description, discount_type, discount_value, 
                   minimum_amount, usage_limit, used_count, valid_from, valid_until
            FROM coupons 
            WHERE code = ?
            AND date('now') BETWEEN valid_from AND valid_until
            "#,
        )
        .bind(code_str)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| DomainError::InvalidCoupon { 
            code: code_str.to_string(), 
            message: format!("Database error: {}", e) 
        })?;

        if let Some(row) = row {
            let discount_type: String = row.get("discount_type");
            let discount_value: i32 = row.get("discount_value");
            let minimum_amount: Option<i32> = row.get("minimum_amount");
            let usage_limit: Option<i32> = row.get("usage_limit");
            let used_count: i32 = row.get("used_count");
            let valid_from_str: String = row.get("valid_from");
            let valid_until_str: String = row.get("valid_until");

            // 日付パース
            let valid_from = DateTime::parse_from_str(&format!("{}T00:00:00+00:00", valid_from_str), "%Y-%m-%dT%H:%M:%S%z")
                .map_err(|e| DomainError::InvalidCoupon { 
                    code: code_str.to_string(), 
                    message: format!("Invalid date format: {}", e) 
                })?
                .with_timezone(&Utc);
            
            let valid_until = DateTime::parse_from_str(&format!("{}T23:59:59+00:00", valid_until_str), "%Y-%m-%dT%H:%M:%S%z")
                .map_err(|e| DomainError::InvalidCoupon { 
                    code: code_str.to_string(), 
                    message: format!("Invalid date format: {}", e) 
                })?
                .with_timezone(&Utc);

            // DiscountPolicyを作成
            let discount_condition = minimum_amount.map(|a| DiscountCondition::MinimumPurchase(Money::from_yen(a as u32)));
            
            let discount_policy = if discount_type == "percentage" {
                DiscountPolicy::new(
                    DiscountType::Percentage(discount_value as u8),
                    discount_condition,
                )
            } else {
                DiscountPolicy::new(
                    DiscountType::FixedAmount(Money::from_yen(discount_value as u32)),
                    discount_condition,
                )
            };

            let coupon = Coupon::new(
                CouponId::from_uuid(
                    Uuid::parse_str(&row.get::<String, _>("id"))
                        .map_err(|e| DomainError::InvalidCoupon { 
                            code: code_str.to_string(), 
                            message: format!("Invalid coupon ID format: {}", e) 
                        })?
                ),
                CouponCode::from_string(row.get::<String, _>("code"))
                    .map_err(|e| DomainError::InvalidCoupon { 
                        code: code_str.to_string(), 
                        message: format!("Invalid coupon code: {}", e) 
                    })?,
                row.get::<String, _>("name"),
                row.get::<Option<String>, _>("description"),
                discount_policy,
                valid_from,
                valid_until,
                usage_limit.map(|l| l as u32),
                used_count as u32,
            );

            Ok(Some(coupon))
        } else {
            Ok(None)
        }
    }

    /// クーポンの使用回数を更新
    async fn update_usage_count(&self, coupon: &Coupon) -> Result<(), DomainError> {
        let new_count = coupon.usage_count() + 1;
        let coupon_id = coupon.id().value().to_string();
        
        sqlx::query(
            r#"
            UPDATE coupons 
            SET used_count = ?, updated_at = datetime('now')
            WHERE id = ?
            "#,
        )
        .bind(new_count as i32)
        .bind(&coupon_id)
        .execute(&self.pool)
        .await
        .map_err(|e| DomainError::InvalidCoupon { 
            code: coupon.code().value().to_string(), 
            message: format!("Failed to update usage count: {}", e) 
        })?;

        Ok(())
    }
}