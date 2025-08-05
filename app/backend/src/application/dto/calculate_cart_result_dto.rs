use crate::domain::{Cart, Money};

/// カートアイテム計算結果DTO
#[derive(Debug, Clone)]
pub struct CalculatedCartItemDto {
    pub sku_id: String,
    pub product_id: String,
    pub product_name: String,
    pub unit_price: Money,
    pub quantity: u32,
    pub subtotal: Money,
}

/// クーポン適用結果DTO
#[derive(Debug, Clone)]
pub struct AppliedCouponDto {
    pub coupon_code: String,
    pub coupon_name: String,
    pub discount_amount: Money,
    pub message: String,
}

/// クーポンエラー詳細DTO
#[derive(Debug, Clone)]
pub struct CouponErrorDto {
    pub coupon_code: Option<String>,
    pub error_message: String,
}

/// カート計算結果DTO
/// CQRS命名規則: CalculateCartCommand の結果
/// すべての計算済みの値を含む
#[derive(Debug, Clone)]
pub struct CalculateCartResultDto {
    pub items: Vec<CalculatedCartItemDto>,
    pub total_quantity: u32,
    pub item_count: usize,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total_with_tax: Money,
    pub is_empty: bool,
    pub shipping_fee: Money,
    pub payment_fee: Money,
    pub applied_coupon: Option<AppliedCouponDto>,
    pub coupon_error: Option<CouponErrorDto>,
}

impl CalculateCartResultDto {
    pub fn from_cart(cart: Cart, coupon_error: Option<CouponErrorDto>) -> Result<Self, String> {
        // カート計算を一括実行
        let calculation = cart
            .calculate()
            .map_err(|e| format!("Failed to calculate cart: {}", e))?;

        // クーポン情報をCartから取得
        let applied_coupon = cart.coupon().map(|coupon| AppliedCouponDto {
            coupon_code: coupon.code().value().to_string(),
            coupon_name: coupon.name().to_string(),
            discount_amount: calculation.discount_amount,
            message: format!("Coupon '{}' applied", coupon.name()),
        });

        // カートアイテムの計算
        let mut items = Vec::new();
        for item in cart.items() {
            let subtotal = item
                .subtotal()
                .map_err(|e| format!("Failed to calculate item subtotal: {}", e))?;

            items.push(CalculatedCartItemDto {
                sku_id: item.sku_id().to_string(),
                product_id: item.product_id().to_string(),
                product_name: item.product_name().value().to_string(),
                unit_price: item.unit_price(),
                quantity: item.quantity(),
                subtotal,
            });
        }

        Ok(Self {
            items,
            total_quantity: cart.total_quantity(),
            item_count: cart.item_count(),
            subtotal: calculation.final_subtotal,
            tax_amount: calculation.tax_amount,
            total_with_tax: calculation.total_with_tax,
            is_empty: cart.is_empty(),
            shipping_fee: calculation.shipping_fee,
            payment_fee: calculation.payment_fee,
            applied_coupon,
            coupon_error,
        })
    }
}
