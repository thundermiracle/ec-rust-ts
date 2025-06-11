#[derive(Debug, Clone, PartialEq)]
pub enum DomainError {
    /// 在庫不足エラー
    InsufficientQuantity { 
        requested: u32, 
        available: u32 
    },
    /// 無効な商品データエラー
    InvalidProductData(String),
}

impl std::fmt::Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::InsufficientQuantity { requested, available } => {
                write!(f, "Insufficient quantity: requested {}, available {}", requested, available)
            }
            DomainError::InvalidProductData(msg) => {
                write!(f, "Invalid product data: {}", msg)
            }
        }
    }
}

impl std::error::Error for DomainError {} 