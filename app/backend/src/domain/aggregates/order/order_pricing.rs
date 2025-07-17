use crate::domain::value_objects::Money;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderPricing {
    pub subtotal: Money,
    pub shipping_fee: Money,
    pub payment_fee: Money,
    pub tax_amount: Money,
    pub total: Money,
}

impl OrderPricing {
    pub fn new(
        subtotal: Money,
        shipping_fee: Money,
        payment_fee: Money,
        tax_amount: Money,
        total: Money,
    ) -> Self {
        OrderPricing {
            subtotal,
            shipping_fee,
            payment_fee,
            tax_amount,
            total,
        }
    }

    pub fn subtotal_yen(&self) -> u32 {
        self.subtotal.amount_in_yen()
    }

    pub fn shipping_fee_yen(&self) -> u32 {
        self.shipping_fee.amount_in_yen()
    }

    pub fn payment_fee_yen(&self) -> u32 {
        self.payment_fee.amount_in_yen()
    }

    pub fn tax_amount_yen(&self) -> u32 {
        self.tax_amount.amount_in_yen()
    }

    pub fn total_yen(&self) -> u32 {
        self.total.amount_in_yen()
    }

    pub fn total_before_tax(&self) -> Result<Money, crate::domain::error::DomainError> {
        self.subtotal.add(self.shipping_fee)?.add(self.payment_fee)
    }

    pub fn verify_calculations(&self) -> bool {
        match self.total_before_tax() {
            Ok(expected_total_before_tax) => {
                let expected_tax = expected_total_before_tax.tax_amount();
                match expected_total_before_tax.add(expected_tax) {
                    Ok(expected_total) => {
                        self.tax_amount == expected_tax && self.total == expected_total
                    }
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_pricing() {
        let subtotal = Money::from_yen(2000);
        let shipping_fee = Money::from_yen(500);
        let payment_fee = Money::from_yen(100);
        let total_before_tax = subtotal
            .add(shipping_fee)
            .unwrap()
            .add(payment_fee)
            .unwrap();
        let tax_amount = total_before_tax.tax_amount();
        let total = total_before_tax.add(tax_amount).unwrap();

        let pricing = OrderPricing::new(subtotal, shipping_fee, payment_fee, tax_amount, total);

        assert_eq!(pricing.subtotal_yen(), 2000);
        assert_eq!(pricing.shipping_fee_yen(), 500);
        assert_eq!(pricing.payment_fee_yen(), 100);
        assert_eq!(pricing.total_before_tax().unwrap(), Money::from_yen(2600));
        assert!(pricing.verify_calculations());
    }

    #[test]
    fn test_invalid_pricing_calculations() {
        let subtotal = Money::from_yen(1000);
        let shipping_fee = Money::from_yen(500);
        let payment_fee = Money::from_yen(100);
        let tax_amount = Money::from_yen(50); // Incorrect tax amount
        let total = Money::from_yen(1650); // Incorrect total

        let pricing = OrderPricing::new(subtotal, shipping_fee, payment_fee, tax_amount, total);

        assert!(!pricing.verify_calculations());
    }

    #[test]
    fn test_correct_tax_calculation() {
        let subtotal = Money::from_yen(1000);
        let shipping_fee = Money::from_yen(500);
        let payment_fee = Money::from_yen(0);
        let total_before_tax = subtotal
            .add(shipping_fee)
            .unwrap()
            .add(payment_fee)
            .unwrap();
        let tax_amount = total_before_tax.tax_amount();
        let total = total_before_tax.add(tax_amount).unwrap();

        let pricing = OrderPricing::new(subtotal, shipping_fee, payment_fee, tax_amount, total);

        // 1500 yen before tax, 10% tax = 150 yen tax, total = 1650 yen
        assert_eq!(pricing.total_before_tax().unwrap(), Money::from_yen(1500));
        assert_eq!(pricing.tax_amount_yen(), 150);
        assert_eq!(pricing.total_yen(), 1650);
        assert!(pricing.verify_calculations());
    }
}
