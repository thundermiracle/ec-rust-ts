use crate::domain::value_objects::Money;

/// 支払い手数料計算のドメインサービス
pub struct PaymentFeeCalculator;

impl PaymentFeeCalculator {
    /// 支払い方法IDに基づいて手数料を計算
    pub fn calculate_fee(payment_method_id: &str, total_amount: Money) -> Money {
        match payment_method_id {
            "cod" => Self::calculate_cod_fee(total_amount),
            "convenience_store" => Money::from_yen(200),
            _ => Money::from_yen(0), // credit_card, bank_transfer は手数料なし
        }
    }

    /// 代引き手数料の計算
    fn calculate_cod_fee(total_amount: Money) -> Money {
        let yen_amount = total_amount.yen();
        
        match yen_amount {
            0..=9999 => Money::from_yen(330),        // 1万円未満
            10000..=29999 => Money::from_yen(440),   // 3万円未満  
            30000..=99999 => Money::from_yen(660),   // 10万円未満
            100000..=299999 => Money::from_yen(1100), // 30万円未満
            _ => Money::from_yen(1650),              // 30万円以上
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cod_fee_calculation() {
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(5000)), Money::from_yen(330));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(15000)), Money::from_yen(440));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(50000)), Money::from_yen(660));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(150000)), Money::from_yen(1100));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(350000)), Money::from_yen(1650));
    }

    #[test]
    fn test_convenience_store_fee() {
        assert_eq!(PaymentFeeCalculator::calculate_fee("convenience_store", Money::from_yen(1000)), Money::from_yen(200));
        assert_eq!(PaymentFeeCalculator::calculate_fee("convenience_store", Money::from_yen(100000)), Money::from_yen(200));
    }

    #[test]
    fn test_no_fee_payment_methods() {
        assert_eq!(PaymentFeeCalculator::calculate_fee("credit_card", Money::from_yen(10000)), Money::from_yen(0));
        assert_eq!(PaymentFeeCalculator::calculate_fee("bank_transfer", Money::from_yen(10000)), Money::from_yen(0));
    }

    #[test]
    fn test_boundary_values() {
        // 境界値のテスト
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(9999)), Money::from_yen(330));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(10000)), Money::from_yen(440));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(29999)), Money::from_yen(440));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(30000)), Money::from_yen(660));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(99999)), Money::from_yen(660));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(100000)), Money::from_yen(1100));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(299999)), Money::from_yen(1100));
        assert_eq!(PaymentFeeCalculator::calculate_fee("cod", Money::from_yen(300000)), Money::from_yen(1650));
    }
}