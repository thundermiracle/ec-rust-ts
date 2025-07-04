use crate::domain::DomainError;

#[derive(Debug)]
pub enum ApplicationError {
    /// ドメインエラー
    Domain(DomainError),
    /// リポジトリエラー
    Repository(RepositoryError),
    /// 商品が見つからない
    ProductNotFound(String),
    /// バリデーションエラー
    Validation(String),
    /// 入力値エラー
    InvalidInput(String),
    /// リソースが見つからない
    NotFound(String),
}

#[derive(Debug)]
pub enum RepositoryError {
    /// データベース接続エラー
    DatabaseConnection(String),
    /// クエリ実行エラー
    QueryExecution(String),
    /// データが見つからない
    NotFound,
    /// その他のエラー
    Unknown(String),
    /// データベースエラー
    DatabaseError(String),
    /// データ変換エラー
    DataConversionError(String),
    /// ドメインエラー
    DomainError(String),
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Domain(err) => write!(f, "Domain error: {}", err),
            ApplicationError::Repository(err) => write!(f, "Repository error: {}", err),
            ApplicationError::ProductNotFound(id) => write!(f, "Product not found: {}", id),
            ApplicationError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ApplicationError::InvalidInput(msg) => write!(f, "Invalid input error: {}", msg),
            ApplicationError::NotFound(msg) => write!(f, "Not found error: {}", msg),
        }
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::DatabaseConnection(msg) => write!(f, "Database connection error: {}", msg),
            RepositoryError::QueryExecution(msg) => write!(f, "Query execution error: {}", msg),
            RepositoryError::NotFound => write!(f, "Data not found"),
            RepositoryError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::DataConversionError(msg) => write!(f, "Data conversion error: {}", msg),
            RepositoryError::DomainError(msg) => write!(f, "Domain error: {}", msg),
        }
    }
}

impl std::error::Error for ApplicationError {}
impl std::error::Error for RepositoryError {}

// DomainErrorからApplicationErrorへの変換
impl From<DomainError> for ApplicationError {
    fn from(err: DomainError) -> Self {
        ApplicationError::Domain(err)
    }
}

// RepositoryErrorからApplicationErrorへの変換
impl From<RepositoryError> for ApplicationError {
    fn from(err: RepositoryError) -> Self {
        ApplicationError::Repository(err)
    }
}

 