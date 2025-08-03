use crate::domain::value_objects::*;

#[derive(Debug, Clone, PartialEq)]
pub enum DiscountCondition {
    MinimumPurchase(Money),
    ProductSpecific(Vec<ProductId>),
    CategorySpecific(Vec<CategoryId>),
}
