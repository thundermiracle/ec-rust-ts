/// EC Rust Backend Library
/// Clean Architecture implementation for E-commerce backend
pub mod application;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod presentation;

// Re-export commonly used items for easier access
pub use error::{Error, Result};
