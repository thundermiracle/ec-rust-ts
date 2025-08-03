use chrono::prelude::*;

use crate::domain::value_objects::{CouponCode, CouponId, DiscountPolicy};

#[derive(Debug, Clone, PartialEq)]
pub struct Coupon {
    id: CouponId,
    code: CouponCode,
    name: String,
    description: Option<String>,
    discount_policy: DiscountPolicy,
    valid_from: DateTime<Utc>,
    valid_until: DateTime<Utc>,
    usage_limit: Option<u32>,
    usage_count: u32,
}

impl Coupon {
    pub fn new(
        id: CouponId,
        code: CouponCode,
        name: String,
        description: Option<String>,
        discount_policy: DiscountPolicy,
        valid_from: DateTime<Utc>,
        valid_until: DateTime<Utc>,
        usage_limit: Option<u32>,
        usage_count: u32,
    ) -> Self {
        Self {
            id,
            code,
            name,
            description,
            discount_policy,
            valid_from,
            valid_until,
            usage_limit,
            usage_count,
        }
    }

    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        self.valid_from <= now && now <= self.valid_until
    }

    pub fn is_valid_usage_limit(&self) -> bool {
        self.usage_limit.is_none() || self.usage_count < self.usage_limit.unwrap()
    }

    // Getters
    pub fn id(&self) -> &CouponId {
        &self.id
    }

    pub fn code(&self) -> &CouponCode {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn discount_policy(&self) -> &DiscountPolicy {
        &self.discount_policy
    }

    pub fn valid_from(&self) -> DateTime<Utc> {
        self.valid_from
    }

    pub fn valid_until(&self) -> DateTime<Utc> {
        self.valid_until
    }

    pub fn usage_limit(&self) -> Option<u32> {
        self.usage_limit
    }

    pub fn usage_count(&self) -> u32 {
        self.usage_count
    }
}
