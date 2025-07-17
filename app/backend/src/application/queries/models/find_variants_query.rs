use crate::domain::SKUId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindVariantsQuery {
    pub sku_ids: Vec<SKUId>,
}

impl FindVariantsQuery {
    pub fn new(sku_ids: Vec<SKUId>) -> Self {
        Self { sku_ids }
    }
}
