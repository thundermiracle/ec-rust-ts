use crate::domain::value_objects::{CouponCode, CouponId, DiscountPolicy, Money};

#[derive(Debug, Clone, PartialEq)]
pub struct CouponInfo {
    pub coupon_id: CouponId,
    pub code: CouponCode,
    pub name: String,
    pub discount_policy: DiscountPolicy,
    pub applied_discount_amount: Money,
}

impl CouponInfo {
    pub fn new(
        coupon_id: CouponId,
        code: CouponCode,
        name: String,
        discount_policy: DiscountPolicy,
        applied_discount_amount: Money,
    ) -> Self {
        CouponInfo {
            coupon_id,
            code,
            name,
            discount_policy,
            applied_discount_amount,
        }
    }

    pub fn coupon_id(&self) -> &CouponId {
        &self.coupon_id
    }

    pub fn code(&self) -> &CouponCode {
        &self.code
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn discount_policy(&self) -> &DiscountPolicy {
        &self.discount_policy
    }

    pub fn applied_discount_amount(&self) -> Money {
        self.applied_discount_amount
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountInfo {
    pub applied_coupons: Vec<CouponInfo>,
    pub total_discount_amount: Money,
}

impl DiscountInfo {
    pub fn new(applied_coupons: Vec<CouponInfo>) -> Self {
        let total_discount_amount = applied_coupons
            .iter()
            .fold(Money::zero(), |acc, coupon| acc.add(coupon.applied_discount_amount).unwrap_or(acc));

        DiscountInfo {
            applied_coupons,
            total_discount_amount,
        }
    }

    pub fn applied_coupons(&self) -> &Vec<CouponInfo> {
        &self.applied_coupons
    }

    pub fn total_discount_amount(&self) -> Money {
        self.total_discount_amount
    }

    pub fn coupon_count(&self) -> usize {
        self.applied_coupons.len()
    }

    pub fn has_coupons(&self) -> bool {
        !self.applied_coupons.is_empty()
    }

    pub fn get_coupon_by_code(&self, code: &CouponCode) -> Option<&CouponInfo> {
        self.applied_coupons.iter().find(|coupon| coupon.code == *code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::value_objects::{DiscountType, Money};

    fn create_test_coupon_info() -> CouponInfo {
        CouponInfo::new(
            CouponId::new(),
            CouponCode::from_string("TEST10".to_string()).unwrap(),
            "Test 10% Off".to_string(),
            DiscountPolicy::unconditional(DiscountType::Percentage(10)),
            Money::from_yen(100),
        )
    }

    #[test]
    fn test_coupon_info_creation() {
        let coupon_info = create_test_coupon_info();
        
        assert_eq!(coupon_info.name(), "Test 10% Off");
        assert_eq!(coupon_info.applied_discount_amount(), Money::from_yen(100));
    }

    #[test]
    fn test_discount_info_with_single_coupon() {
        let coupon_info = create_test_coupon_info();
        let discount_info = DiscountInfo::new(vec![coupon_info]);

        assert_eq!(discount_info.coupon_count(), 1);
        assert_eq!(discount_info.total_discount_amount(), Money::from_yen(100));
        assert!(discount_info.has_coupons());
    }

    #[test]
    fn test_discount_info_with_multiple_coupons() {
        let coupon1 = create_test_coupon_info();
        let mut coupon2 = create_test_coupon_info();
        coupon2.code = CouponCode::from_string("TEST20".to_string()).unwrap();
        coupon2.applied_discount_amount = Money::from_yen(200);

        let discount_info = DiscountInfo::new(vec![coupon1, coupon2]);

        assert_eq!(discount_info.coupon_count(), 2);
        assert_eq!(discount_info.total_discount_amount(), Money::from_yen(300));
    }

    #[test]
    fn test_discount_info_empty() {
        let discount_info = DiscountInfo::new(vec![]);

        assert_eq!(discount_info.coupon_count(), 0);
        assert_eq!(discount_info.total_discount_amount(), Money::zero());
        assert!(!discount_info.has_coupons());
    }

    #[test]
    fn test_get_coupon_by_code() {
        let coupon_info = create_test_coupon_info();
        let code = coupon_info.code.clone();
        let discount_info = DiscountInfo::new(vec![coupon_info]);

        let found_coupon = discount_info.get_coupon_by_code(&code);
        assert!(found_coupon.is_some());
        assert_eq!(found_coupon.unwrap().name(), "Test 10% Off");

        let not_found = discount_info.get_coupon_by_code(&CouponCode::from_string("NOTFOUND".to_string()).unwrap());
        assert!(not_found.is_none());
    }
}