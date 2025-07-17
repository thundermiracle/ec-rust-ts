#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    /// 在庫不足エラー
    InsufficientStock { requested: u32, available: u32 },
    /// 無効な商品データエラー
    InvalidProductData(String),
    /// 無効な商品名エラー
    InvalidProductName(String),
    /// 無効な価格エラー
    InvalidPrice(String),
    /// 無効なSKUコードエラー
    InvalidSKUCode(String),
    /// 無効な在庫エラー
    InvalidStock(String),
    /// ビジネスルール違反エラー
    BusinessRuleViolation(String),
    /// 無効な商品状態エラー
    InvalidProductState(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::InsufficientStock {
                requested,
                available,
            } => {
                write!(
                    f,
                    "Insufficient stock: requested {}, available {}",
                    requested, available
                )
            }
            DomainError::InvalidProductData(msg) => {
                write!(f, "Invalid product data: {}", msg)
            }
            DomainError::InvalidProductName(msg) => {
                write!(f, "Invalid product name: {}", msg)
            }
            DomainError::InvalidPrice(msg) => {
                write!(f, "Invalid price: {}", msg)
            }
            DomainError::InvalidSKUCode(msg) => {
                write!(f, "Invalid SKU code: {}", msg)
            }
            DomainError::InvalidStock(msg) => {
                write!(f, "Invalid stock: {}", msg)
            }
            DomainError::BusinessRuleViolation(msg) => {
                write!(f, "Business rule violation: {}", msg)
            }
            DomainError::InvalidProductState(msg) => {
                write!(f, "Invalid product state: {}", msg)
            }
        }
    }
}

impl std::error::Error for DomainError {}
