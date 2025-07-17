use crate::domain::error::DomainError;
use std::fmt;

/// 日本円金額値オブジェクト
/// 円単位で金額を管理し、計算は切り上げ（ceiling）を使用
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money {
    /// 金額（円単位）
    amount_in_yen: u32,
}

impl Money {
    /// ゼロ円を作成
    pub fn zero() -> Self {
        Self::from_yen(0)
    }

    /// 円単位で金額を作成
    pub fn from_yen(yen: u32) -> Self {
        Self { amount_in_yen: yen }
    }

    /// 文字列から金額を作成（例: "1234" -> 1234円）
    pub fn from_string(amount_str: &str) -> Result<Self, DomainError> {
        let trimmed = amount_str.trim();
        let yen: u32 = trimmed
            .parse()
            .map_err(|_| DomainError::InvalidProductData("Invalid yen amount".to_string()))?;
        Ok(Self::from_yen(yen))
    }

    /// 円単位の金額を取得
    pub fn yen(&self) -> u32 {
        self.amount_in_yen
    }

    /// 円単位の金額を取得（別名）
    pub fn amount_in_yen(&self) -> u32 {
        self.amount_in_yen
    }

    /// 浮動小数点数として取得（計算目的）
    pub fn as_float(&self) -> f64 {
        self.amount_in_yen as f64
    }

    /// 金額が0かどうかを判定
    pub fn is_zero(&self) -> bool {
        self.amount_in_yen == 0
    }

    /// 正の金額かどうかを判定
    pub fn is_positive(&self) -> bool {
        self.amount_in_yen > 0
    }

    /// 加算
    pub fn add(&self, other: Money) -> Result<Money, DomainError> {
        let result = self
            .amount_in_yen
            .checked_add(other.amount_in_yen)
            .ok_or_else(|| {
                DomainError::InvalidProductData("Money overflow in addition".to_string())
            })?;
        Ok(Money::from_yen(result))
    }

    /// 減算
    pub fn subtract(&self, other: Money) -> Result<Money, DomainError> {
        if other.amount_in_yen > self.amount_in_yen {
            return Err(DomainError::InvalidProductData(
                "Cannot subtract larger amount from smaller amount".to_string(),
            ));
        }
        Ok(Money::from_yen(self.amount_in_yen - other.amount_in_yen))
    }

    /// 乗算
    pub fn multiply(&self, multiplier: u32) -> Result<Money, DomainError> {
        let result = self.amount_in_yen.checked_mul(multiplier).ok_or_else(|| {
            DomainError::InvalidProductData("Money overflow in multiplication".to_string())
        })?;
        Ok(Money::from_yen(result))
    }

    /// パーセンテージ計算（切り上げ使用）
    pub fn percentage(&self, percentage: f64) -> Result<Money, DomainError> {
        if percentage < 0.0 || percentage > 1.0 {
            return Err(DomainError::InvalidProductData(
                "Percentage must be between 0.0 and 1.0".to_string(),
            ));
        }

        let result = (self.amount_in_yen as f64 * percentage).ceil() as u32;
        Ok(Money::from_yen(result))
    }

    /// 割引を適用（切り上げ使用）
    pub fn apply_discount(&self, discount_percentage: u8) -> Result<Money, DomainError> {
        if discount_percentage > 100 {
            return Err(DomainError::InvalidProductData(
                "Discount percentage cannot exceed 100".to_string(),
            ));
        }

        let discount_ratio = discount_percentage as f64 / 100.0;
        let discount_amount = self.percentage(discount_ratio)?;
        self.subtract(discount_amount)
    }

    /// 税込価格を計算（消費税10%、切り上げ）
    pub fn with_tax(&self) -> Money {
        let tax_amount = (self.amount_in_yen as f64 * 0.10).ceil() as u32;
        Money::from_yen(self.amount_in_yen + tax_amount)
    }

    /// 消費税額を計算（10%、切り上げ）
    pub fn tax_amount(&self) -> Money {
        let tax = (self.amount_in_yen as f64 * 0.10).ceil() as u32;
        Money::from_yen(tax)
    }

    /// 日本円フォーマット
    pub fn format_jpy(&self) -> String {
        format!("¥{}", self.amount_in_yen)
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_jpy())
    }
}

// 算術演算子のオーバーロード
impl std::ops::Add for Money {
    type Output = Result<Money, DomainError>;

    fn add(self, other: Money) -> Self::Output {
        Money::add(&self, other)
    }
}

impl std::ops::Sub for Money {
    type Output = Result<Money, DomainError>;

    fn sub(self, other: Money) -> Self::Output {
        Money::subtract(&self, other)
    }
}

impl std::ops::Mul<u32> for Money {
    type Output = Result<Money, DomainError>;

    fn mul(self, multiplier: u32) -> Self::Output {
        Money::multiply(&self, multiplier)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_money_from_yen() {
        let money = Money::from_yen(1000);
        assert_eq!(money.yen(), 1000);
    }

    #[test]
    fn create_money_from_string() {
        assert_eq!(Money::from_string("1500").unwrap().yen(), 1500);
        assert_eq!(Money::from_string("0").unwrap().yen(), 0);
    }

    #[test]
    fn reject_invalid_string() {
        assert!(Money::from_string("abc").is_err());
        assert!(Money::from_string("12.34").is_err()); // 日本円は整数のみ
    }

    #[test]
    fn arithmetic_operations() {
        let money1 = Money::from_yen(1000);
        let money2 = Money::from_yen(500);

        let sum = (money1 + money2).unwrap();
        assert_eq!(sum.yen(), 1500);

        let diff = (money1 - money2).unwrap();
        assert_eq!(diff.yen(), 500);

        let product = (money1 * 3).unwrap();
        assert_eq!(product.yen(), 3000);
    }

    #[test]
    fn percentage_calculations_with_ceiling() {
        let money = Money::from_yen(1000);
        let ten_percent = money.percentage(0.10).unwrap();
        assert_eq!(ten_percent.yen(), 100); // 1000 * 0.1 = 100.0 -> ceil = 100

        // 切り上げテスト
        let money2 = Money::from_yen(999);
        let ten_percent2 = money2.percentage(0.10).unwrap();
        assert_eq!(ten_percent2.yen(), 100); // 999 * 0.1 = 99.9 -> ceil = 100
    }

    #[test]
    fn discount_application() {
        let money = Money::from_yen(1000);
        let discounted = money.apply_discount(20).unwrap(); // 20% off
        assert_eq!(discounted.yen(), 800); // 1000 - 200 = 800
    }

    #[test]
    fn tax_calculations() {
        let money = Money::from_yen(1000);
        let with_tax = money.with_tax();
        assert_eq!(with_tax.yen(), 1100); // 1000 + 100 = 1100

        let tax_amount = money.tax_amount();
        assert_eq!(tax_amount.yen(), 100);

        // 切り上げテスト
        let money2 = Money::from_yen(999);
        let tax_amount2 = money2.tax_amount();
        assert_eq!(tax_amount2.yen(), 100); // 999 * 0.1 = 99.9 -> ceil = 100
    }

    #[test]
    fn formatting() {
        let money = Money::from_yen(12345);
        assert_eq!(money.format_jpy(), "¥12345");
        assert_eq!(money.to_string(), "¥12345");
    }

    #[test]
    fn comparison_operations() {
        let money1 = Money::from_yen(1000);
        let money2 = Money::from_yen(2000);
        let money3 = Money::from_yen(1000);

        assert!(money1 < money2);
        assert!(money2 > money1);
        assert_eq!(money1, money3);
    }

    #[test]
    fn check_zero_and_positive() {
        let zero = Money::from_yen(0);
        let positive = Money::from_yen(100);

        assert!(zero.is_zero());
        assert!(!zero.is_positive());
        assert!(!positive.is_zero());
        assert!(positive.is_positive());
    }
}
