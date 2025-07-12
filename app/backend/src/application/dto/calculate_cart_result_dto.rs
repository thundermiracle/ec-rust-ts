use crate::domain::{Cart, Money};

/// カート計算結果DTO
/// CQRS命名規則: CalculateCartCommand の結果
#[derive(Debug, Clone)]
pub struct CalculateCartResultDto {
    pub cart: Cart,
    pub shipping_fee: Money,
    pub payment_fee: Money,
}

impl CalculateCartResultDto {
    pub fn new(cart: Cart, shipping_fee: Money, payment_fee: Money) -> Self {
        Self {
            cart,
            shipping_fee,
            payment_fee,
        }
    }
}