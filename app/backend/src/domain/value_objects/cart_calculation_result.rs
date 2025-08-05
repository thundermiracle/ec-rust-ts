use crate::domain::value_objects::Money;

/// カート計算結果
/// 一括計算による効率的なデータ提供
#[derive(Debug, Clone, PartialEq)]
pub struct CartCalculationResult {
    /// 原価小計（割引適用前）
    pub original_subtotal: Money,
    /// 割引額
    pub discount_amount: Money,
    /// 最終小計（割引適用後）
    pub final_subtotal: Money,
    /// 税額
    pub tax_amount: Money,
    /// 税込み合計
    pub total_with_tax: Money,
    /// 配送料
    pub shipping_fee: Money,
    /// 支払い手数料
    pub payment_fee: Money,
    /// 最終合計（税込み + 配送料 + 支払い手数料）
    pub grand_total: Money,
}

impl CartCalculationResult {
    pub fn new(
        original_subtotal: Money,
        discount_amount: Money,
        final_subtotal: Money,
        tax_amount: Money,
        total_with_tax: Money,
        shipping_fee: Money,
        payment_fee: Money,
        grand_total: Money,
    ) -> Self {
        Self {
            original_subtotal,
            discount_amount,
            final_subtotal,
            tax_amount,
            total_with_tax,
            shipping_fee,
            payment_fee,
            grand_total,
        }
    }
}