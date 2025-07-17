use crate::domain::error::DomainError;
use crate::domain::value_objects::*;

/// 支払い方法エンティティ
#[derive(Debug, Clone, PartialEq)]
pub struct PaymentMethod {
    id: String,
    name: String,
    description: String,
    is_active: bool,
    sort_order: u32,
}

impl PaymentMethod {
    /// 新しい支払い方法を作成
    pub fn new(
        id: String,
        name: String,
        description: String,
        is_active: bool,
        sort_order: u32,
    ) -> Result<Self, DomainError> {
        if id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Payment method ID cannot be empty".to_string(),
            ));
        }

        if name.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Payment method name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            id,
            name,
            description,
            is_active,
            sort_order,
        })
    }

    /// 支払い手数料を計算
    pub fn calculate_fee(&self, cart_total: Money) -> Result<Money, DomainError> {
        if !self.is_active {
            return Err(DomainError::InvalidProductData(
                "Payment method is not active".to_string(),
            ));
        }

        match self.id.as_str() {
            "cod" => self.calculate_cod_fee(cart_total),
            "convenience_store" => Ok(Money::from_yen(200)),
            "credit_card" | "bank_transfer" => Ok(Money::from_yen(0)),
            _ => Ok(Money::from_yen(0)),
        }
    }

    /// 代引き手数料の計算
    fn calculate_cod_fee(&self, total_amount: Money) -> Result<Money, DomainError> {
        let yen_amount = total_amount.yen();

        let fee_amount = match yen_amount {
            0..=9999 => 330,         // 1万円未満
            10000..=29999 => 440,    // 3万円未満
            30000..=99999 => 660,    // 10万円未満
            100000..=299999 => 1100, // 30万円未満
            _ => 1650,               // 30万円以上
        };

        Ok(Money::from_yen(fee_amount))
    }

    /// 支払い方法が利用可能かチェック
    pub fn is_available(&self) -> bool {
        self.is_active
    }

    /// 代引きかどうか
    pub fn is_cod(&self) -> bool {
        self.id == "cod"
    }

    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn sort_order(&self) -> u32 {
        self.sort_order
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_payment_method() {
        let method = PaymentMethod::new(
            "cod".to_string(),
            "代金引換".to_string(),
            "商品お届け時にお支払い".to_string(),
            true,
            1,
        )
        .unwrap();

        assert_eq!(method.id(), "cod");
        assert_eq!(method.name(), "代金引換");
        assert!(method.is_active());
        assert!(method.is_cod());
    }

    #[test]
    fn calculate_cod_fee_tiers() {
        let cod_method = PaymentMethod::new(
            "cod".to_string(),
            "代金引換".to_string(),
            "商品お届け時にお支払い".to_string(),
            true,
            1,
        )
        .unwrap();

        // 1万円未満: 330円
        assert_eq!(
            cod_method.calculate_fee(Money::from_yen(5000)).unwrap(),
            Money::from_yen(330)
        );

        // 3万円未満: 440円
        assert_eq!(
            cod_method.calculate_fee(Money::from_yen(15000)).unwrap(),
            Money::from_yen(440)
        );

        // 10万円未満: 660円
        assert_eq!(
            cod_method.calculate_fee(Money::from_yen(50000)).unwrap(),
            Money::from_yen(660)
        );

        // 30万円未満: 1100円
        assert_eq!(
            cod_method.calculate_fee(Money::from_yen(150000)).unwrap(),
            Money::from_yen(1100)
        );

        // 30万円以上: 1650円
        assert_eq!(
            cod_method.calculate_fee(Money::from_yen(400000)).unwrap(),
            Money::from_yen(1650)
        );
    }

    #[test]
    fn calculate_convenience_store_fee() {
        let convenience_method = PaymentMethod::new(
            "convenience_store".to_string(),
            "コンビニ決済".to_string(),
            "コンビニでお支払い".to_string(),
            true,
            2,
        )
        .unwrap();

        assert_eq!(
            convenience_method
                .calculate_fee(Money::from_yen(10000))
                .unwrap(),
            Money::from_yen(200)
        );
    }

    #[test]
    fn calculate_fee_fails_for_inactive_method() {
        let inactive_method = PaymentMethod::new(
            "cod".to_string(),
            "代金引換".to_string(),
            "商品お届け時にお支払い".to_string(),
            false,
            1,
        )
        .unwrap();

        let result = inactive_method.calculate_fee(Money::from_yen(10000));
        assert!(result.is_err());
    }
}
