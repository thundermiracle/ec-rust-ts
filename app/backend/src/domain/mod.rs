pub mod entities;
pub mod value_objects;
pub mod error;
pub mod aggregates;

pub use error::DomainError;
pub use entities::*;
pub use value_objects::*;
pub use aggregates::*;
