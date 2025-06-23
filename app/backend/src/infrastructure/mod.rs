/// Frameworks & Drivers Layer
/// Uncle Bob's Clean Architecture 最外層
/// Web frameworks, databases, external APIs, dependency injection など
pub mod database;
pub mod di;

// メインモジュールからのexport
pub use di::{Container, get_container}; 