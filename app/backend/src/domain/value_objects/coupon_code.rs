use crate::domain::error::DomainError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CouponCode(String);

impl CouponCode {
    const MAX_LENGTH: usize = 10;

    pub fn from_string(value: String) -> Result<Self, DomainError> {
        if value.is_empty() {
            return Err(DomainError::InvalidCoupon {
                code: value,
                message: "Coupon code cannot be empty".to_string(),
            });
        }

        if value.len() > Self::MAX_LENGTH {
            return Err(DomainError::InvalidCoupon {
                code: value,
                message: format!("Coupon code cannot exceed {} characters", Self::MAX_LENGTH),
            });
        }

        Ok(Self(value))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}
