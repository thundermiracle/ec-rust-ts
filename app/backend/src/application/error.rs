use crate::domain::DomainError;
use crate::application::queries::QueryError;

#[derive(Debug)]
pub enum ApplicationError {
    /// ドメインエラー
    Domain(DomainError),
    /// リポジトリエラー
    Repository(RepositoryError),
    /// 商品が見つからない
    ProductNotFound(u32),
    /// バリデーションエラー
    Validation(String),
    /// クエリマッピングエラー
    QueryMapping(QueryError),
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
}

impl std::fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::Domain(err) => write!(f, "Domain error: {}", err),
            ApplicationError::Repository(err) => write!(f, "Repository error: {}", err),
            ApplicationError::ProductNotFound(id) => write!(f, "Product not found: {}", id),
            ApplicationError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ApplicationError::QueryMapping(err) => write!(f, "Query mapping error: {}", err),
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

// QueryErrorからApplicationErrorへの変換
impl From<QueryError> for ApplicationError {
    fn from(err: QueryError) -> Self {
        ApplicationError::QueryMapping(err)
    }
} 