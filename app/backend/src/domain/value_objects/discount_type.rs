use crate::domain::value_objects::Money;

#[derive(Debug, Clone, PartialEq)]
pub enum DiscountType {
    FixedAmount(Money),
    Percentage(u8),
}
