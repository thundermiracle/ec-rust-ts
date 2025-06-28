use serde::{Serialize, Deserialize};
use crate::domain::models::SKUId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindVariantsQuery {
    pub sku_ids: Vec<SKUId>,
}

impl FindVariantsQuery {
    pub fn new(sku_ids: Vec<SKUId>) -> Self {
        Self { sku_ids }
    }
} 