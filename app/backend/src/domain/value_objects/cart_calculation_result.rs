use crate::domain::value_objects::Money;

/// カート計算結果
/// 一括計算による効率的なデータ提供
#[derive(Debug, Clone, PartialEq)]
pub struct CartCalculationResult {
    /// 原価小計（割引適用前）
    pub original_subtotal: Money,
    /// 割引額
    pub discount_amount: Money,
    /// 最終小計（割引適用後、税抜き）
    pub subtotal: Money,
    /// 税額
    pub tax_amount: Money,
    /// 小計税込み（手数料抜き）
    pub subtotal_with_tax: Money,
    /// 配送料
    pub shipping_fee: Money,
    /// 支払い手数料
    pub payment_fee: Money,
    /// 最終支払い総額（全て込み）
    pub total: Money,
}

impl CartCalculationResult {
    pub fn new(
        original_subtotal: Money,
        discount_amount: Money,
        subtotal: Money,
        tax_amount: Money,
        subtotal_with_tax: Money,
        shipping_fee: Money,
        payment_fee: Money,
        total: Money,
    ) -> Self {
        Self {
            original_subtotal,
            discount_amount,
            subtotal,
            tax_amount,
            subtotal_with_tax,
            shipping_fee,
            payment_fee,
            total,
        }
    }
}