use crate::domain::entities::Coupon;
use crate::domain::error::DomainError;
use crate::domain::value_objects::{
    DiscountCondition, DiscountType, Money, ProductId, PurchaseInfo,
};

/// クーポン割引計算サービス
/// クーポンの適用可能性チェックと割引金額計算を行う
pub struct CouponDiscountService;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountResult {
    /// 割引金額
    pub discount_amount: Money,
    /// 割引後の金額
    pub discounted_amount: Money,
    /// 適用されたクーポンのメッセージ
    pub message: String,
}

impl CouponDiscountService {
    /// クーポンを購入情報に適用し、割引結果を計算
    pub fn apply_coupon(
        coupon: &Coupon,
        purchase_info: &PurchaseInfo,
    ) -> Result<DiscountResult, DomainError> {
        // クーポンの有効性をチェック
        Self::validate_coupon(coupon)?;

        // 適用条件をチェック
        Self::check_conditions(coupon, purchase_info)?;

        // 割引金額を計算
        let discount_amount = Self::calculate_discount(coupon, purchase_info)?;

        // 割引後の金額を計算
        let original_amount = purchase_info.subtotal();
        let discounted_amount = original_amount.subtract(discount_amount)?;

        let message = format!(
            "クーポン「{}」が適用されました。割引額: {}",
            coupon.name(),
            discount_amount.format_jpy()
        );

        Ok(DiscountResult {
            discount_amount,
            discounted_amount,
            message,
        })
    }

    /// クーポンの基本的な有効性をチェック
    fn validate_coupon(coupon: &Coupon) -> Result<(), DomainError> {
        if !coupon.is_valid() {
            return Err(DomainError::InvalidCoupon {
                code: coupon.code().value().to_string(),
                message: "クーポンの有効期限が切れています".to_string(),
            });
        }

        if !coupon.is_valid_usage_limit() {
            return Err(DomainError::InvalidCoupon {
                code: coupon.code().value().to_string(),
                message: "クーポンの使用回数が上限に達しています".to_string(),
            });
        }

        Ok(())
    }

    /// クーポンの適用条件をチェック
    fn check_conditions(coupon: &Coupon, purchase_info: &PurchaseInfo) -> Result<(), DomainError> {
        if let Some(condition) = coupon.discount_policy().discount_condition() {
            match condition {
                DiscountCondition::MinimumPurchase(minimum_amount) => {
                    if !purchase_info.meets_minimum_amount(*minimum_amount) {
                        return Err(DomainError::InvalidCoupon {
                            code: coupon.code().value().to_string(),
                            message: format!(
                                "最低購入金額{}に満たないため、クーポンを適用できません",
                                minimum_amount.format_jpy()
                            ),
                        });
                    }
                }
                DiscountCondition::ProductSpecific(product_ids) => {
                    if !Self::has_target_products(purchase_info, product_ids) {
                        return Err(DomainError::InvalidCoupon {
                            code: coupon.code().value().to_string(),
                            message: "対象商品がカートに含まれていません".to_string(),
                        });
                    }
                }
                DiscountCondition::CategorySpecific(_category_ids) => {
                    // カテゴリ条件は今回は実装をスキップ
                    // 実装する場合は、商品情報からカテゴリを取得する必要がある
                    return Err(DomainError::InvalidCoupon {
                        code: coupon.code().value().to_string(),
                        message: "カテゴリ指定のクーポンは現在サポートされていません".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// 購入情報に対象商品が含まれているかチェック
    fn has_target_products(purchase_info: &PurchaseInfo, target_product_ids: &[ProductId]) -> bool {
        target_product_ids
            .iter()
            .any(|product_id| purchase_info.contains_product(product_id))
    }

    /// 割引金額を計算
    fn calculate_discount(
        coupon: &Coupon,
        purchase_info: &PurchaseInfo,
    ) -> Result<Money, DomainError> {
        let base_amount = purchase_info.subtotal();

        match coupon.discount_policy().discount_type() {
            DiscountType::FixedAmount(amount) => {
                // 固定金額割引の場合、割引額が商品金額を超えないようにする
                if *amount > base_amount {
                    Ok(base_amount)
                } else {
                    Ok(*amount)
                }
            }
            DiscountType::Percentage(percentage) => {
                // パーセンテージ割引の場合
                base_amount.apply_discount(*percentage)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::aggregates::CartItem;
    use crate::domain::entities::Coupon;
    use crate::domain::value_objects::{
        CouponCode, CouponId, DiscountCondition, DiscountPolicy, DiscountType, ProductId,
        ProductName, SKUId,
    };
    use chrono::prelude::*;
    use uuid::Uuid;

    fn create_test_cart_item(name: &str, price: u32, quantity: u32) -> CartItem {
        CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            ProductId::from_uuid(Uuid::new_v4()),
            ProductName::new(name.to_string()).unwrap(),
            Money::from_yen(price),
            quantity,
        )
        .unwrap()
    }

    fn create_valid_coupon(
        code: &str,
        name: &str,
        discount_type: DiscountType,
        condition: Option<DiscountCondition>,
    ) -> Coupon {
        Coupon::new(
            CouponId::new(),
            CouponCode::from_string(code.to_string()).unwrap(),
            name.to_string(),
            None,
            DiscountPolicy::new(discount_type, condition),
            Utc::now() - chrono::Duration::days(1), // Valid from yesterday
            Utc::now() + chrono::Duration::days(30), // Valid until 30 days from now
            Some(100),
            0,
        )
    }

    #[test]
    fn apply_percentage_discount_coupon() {
        let coupon = create_valid_coupon("SAVE20", "20% Off", DiscountType::Percentage(20), None);

        let items = vec![create_test_cart_item("Product", 1000, 2)];
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info).unwrap();

        assert_eq!(result.discount_amount.yen(), 400); // 20% of 2000
        assert_eq!(result.discounted_amount.yen(), 1600); // 2000 - 400
        assert!(result.message.contains("20% Off"));
    }

    #[test]
    fn apply_fixed_amount_discount_coupon() {
        let coupon = create_valid_coupon(
            "SAVE500",
            "500円 Off",
            DiscountType::FixedAmount(Money::from_yen(500)),
            None,
        );

        let items = vec![create_test_cart_item("Product", 1000, 2)];
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info).unwrap();

        assert_eq!(result.discount_amount.yen(), 500);
        assert_eq!(result.discounted_amount.yen(), 1500); // 2000 - 500
    }

    #[test]
    fn fixed_amount_discount_cannot_exceed_subtotal() {
        let coupon = create_valid_coupon(
            "SAVE3000",
            "3000円 Off",
            DiscountType::FixedAmount(Money::from_yen(3000)),
            None,
        );

        let items = vec![create_test_cart_item("Product", 1000, 2)];
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info).unwrap();

        assert_eq!(result.discount_amount.yen(), 2000); // Cannot exceed subtotal
        assert_eq!(result.discounted_amount.yen(), 0);
    }

    #[test]
    fn apply_coupon_with_minimum_purchase_condition() {
        let coupon = create_valid_coupon(
            "MIN5000",
            "10% Off (Min 5000円)",
            DiscountType::Percentage(10),
            Some(DiscountCondition::MinimumPurchase(Money::from_yen(5000))),
        );

        let items = vec![create_test_cart_item("Product", 3000, 2)]; // 6000円
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info).unwrap();

        assert_eq!(result.discount_amount.yen(), 600); // 10% of 6000
        assert_eq!(result.discounted_amount.yen(), 5400);
    }

    #[test]
    fn fail_minimum_purchase_condition() {
        let coupon = create_valid_coupon(
            "MIN5000",
            "10% Off (Min 5000円)",
            DiscountType::Percentage(10),
            Some(DiscountCondition::MinimumPurchase(Money::from_yen(5000))),
        );

        let items = vec![create_test_cart_item("Product", 2000, 1)]; // 2000円
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info);

        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::InvalidCoupon { message, .. } => {
                assert!(message.contains("最低購入金額"));
            }
            _ => panic!("Expected InvalidCoupon error"),
        }
    }

    #[test]
    fn apply_coupon_with_product_specific_condition() {
        let product_id = ProductId::from_uuid(Uuid::new_v4());
        let target_product_ids = vec![product_id.clone()];

        let coupon = create_valid_coupon(
            "PRODUCT20",
            "Product Specific 20% Off",
            DiscountType::Percentage(20),
            Some(DiscountCondition::ProductSpecific(target_product_ids)),
        );

        let item = CartItem::new(
            SKUId::from_uuid(Uuid::new_v4()),
            product_id,
            ProductName::new("Target Product".to_string()).unwrap(),
            Money::from_yen(1000),
            1,
        )
        .unwrap();

        let subtotal = Money::from_yen(1000); // 1000 * 1
        let purchase_info = PurchaseInfo::new(vec![item], subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info).unwrap();

        assert_eq!(result.discount_amount.yen(), 200); // 20% of 1000
    }

    #[test]
    fn fail_product_specific_condition() {
        let target_product_id = ProductId::from_uuid(Uuid::new_v4());
        let target_product_ids = vec![target_product_id];

        let coupon = create_valid_coupon(
            "PRODUCT20",
            "Product Specific 20% Off",
            DiscountType::Percentage(20),
            Some(DiscountCondition::ProductSpecific(target_product_ids)),
        );

        let items = vec![create_test_cart_item("Other Product", 1000, 1)];
        let subtotal = Money::from_yen(2000); // 1000 * 2
        let purchase_info = PurchaseInfo::new(items, subtotal, None, None);

        let result = CouponDiscountService::apply_coupon(&coupon, &purchase_info);

        assert!(result.is_err());
        match result.unwrap_err() {
            DomainError::InvalidCoupon { message, .. } => {
                assert!(message.contains("対象商品がカートに含まれていません"));
            }
            _ => panic!("Expected InvalidCoupon error"),
        }
    }
}
