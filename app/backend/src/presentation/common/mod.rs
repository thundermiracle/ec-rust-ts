/// 共通のプレゼンテーション層コンポーネント
/// Clean Architecture: Interface Adapters層の共通機能
pub mod extractors;
pub mod responses;
pub mod validators;

// 共通レスポンス型のエクスポート
pub use responses::ErrorResponse;
