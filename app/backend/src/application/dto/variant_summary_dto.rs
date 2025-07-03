use serde::{Serialize, Deserialize};
use crate::domain::SKUId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantSummaryDTO {
    pub sku_id: SKUId,
    pub price: i32,
    pub sale_price: Option<i32>,
    pub image: Option<String>,
    pub material: Option<String>,
    pub dimensions: Option<String>,
}

impl VariantSummaryDTO {
    pub fn new(
        sku_id: SKUId,
        price: i32,
        sale_price: Option<i32>,
        image: Option<String>,
        material: Option<String>,
        dimensions: Option<String>,
    ) -> Self {
        Self {
            sku_id,
            price,
            sale_price,
            image,
            material,
            dimensions,
        }
    }
} 