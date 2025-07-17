use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description(String);

impl Description {
    pub fn new(description: String) -> Self {
        Self(description.trim().to_string())
    }

    pub fn value(&self) -> &str {
        &self.0
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
