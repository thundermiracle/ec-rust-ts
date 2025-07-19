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
    /// 外部キー制約エラー
    ForeignKeyConstraint {
        /// 制約に関連するテーブル/フィールド
        field: String,
        /// 内部エラー詳細
        message: String,
    },
    /// データ変換エラー
    DataConversionError(String),
    /// データベースエラー（他のリポジトリとの互換性のため）
    DatabaseError(String),
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
            ApplicationError::InvalidInput(msg) => write!(f, "Invalid input error: {}", msg),
            ApplicationError::NotFound(msg) => write!(f, "Not found error: {}", msg),
        }
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::DatabaseConnection(msg) => {
                write!(f, "Database connection error: {}", msg)
            }
            RepositoryError::QueryExecution(msg) => write!(f, "Query execution error: {}", msg),
            RepositoryError::NotFound => write!(f, "Data not found"),
            RepositoryError::ForeignKeyConstraint { field, message } => {
                write!(f, "Foreign key constraint error in {}: {}", field, message)
            }
            RepositoryError::DataConversionError(msg) => {
                write!(f, "Data conversion error: {}", msg)
            }
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::Unknown(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl RepositoryError {
    /// ユーザー向けメッセージを取得（集約された変換ロジック）
    pub fn to_user_message(&self) -> String {
        match self {
            RepositoryError::DatabaseConnection(_) => {
                "一時的な問題が発生しました。しばらくしてから再度お試しください".to_string()
            }
            RepositoryError::QueryExecution(_) => {
                "一時的な問題が発生しました。しばらくしてから再度お試しください".to_string()
            }
            RepositoryError::NotFound => "データが見つかりません".to_string(),
            RepositoryError::ForeignKeyConstraint { field, .. } => match field.as_str() {
                "sku_id" => "指定された商品SKUが見つかりません。商品を確認してください".to_string(),
                "shipping_method_id" => {
                    "指定された配送方法が見つかりません。配送方法を確認してください".to_string()
                }
                "payment_method_id" => {
                    "指定された支払い方法が見つかりません。支払い方法を確認してください".to_string()
                }
                _ => "指定されたデータが見つかりません。入力内容を確認してください".to_string(),
            },
            RepositoryError::DataConversionError(_) => {
                "データの処理中に問題が発生しました。しばらくしてから再度お試しください".to_string()
            }
            RepositoryError::DatabaseError(_) => {
                "一時的な問題が発生しました。しばらくしてから再度お試しください".to_string()
            }
            RepositoryError::Unknown(_) => {
                "一時的な問題が発生しました。しばらくしてから再度お試しください".to_string()
            }
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
