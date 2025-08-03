use crate::domain::value_objects::{
    discount_condition::DiscountCondition, discount_type::DiscountType,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DiscountPolicy {
    discount_type: DiscountType,
    discount_condition: Option<DiscountCondition>,
}

impl DiscountPolicy {
    pub fn new(discount_type: DiscountType, discount_condition: Option<DiscountCondition>) -> Self {
        Self {
            discount_type,
            discount_condition,
        }
    }

    pub fn unconditional(discount_type: DiscountType) -> Self {
        Self {
            discount_type,
            discount_condition: None,
        }
    }

    pub fn has_condition(&self) -> bool {
        self.discount_condition.is_some()
    }

    // Getters
    pub fn discount_type(&self) -> &DiscountType {
        &self.discount_type
    }

    pub fn discount_condition(&self) -> Option<&DiscountCondition> {
        self.discount_condition.as_ref()
    }
}
