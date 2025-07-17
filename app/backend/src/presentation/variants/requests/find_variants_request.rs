use crate::application::queries::models::FindVariantsQuery;
use crate::domain::SKUId;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct FindVariantsRequest {
    #[serde(rename = "skuIds")]
    pub sku_ids: Vec<String>,
}

impl FindVariantsRequest {
    pub fn to_query(&self) -> Result<FindVariantsQuery, String> {
        let mut sku_id_objects = Vec::new();

        for sku_id_str in &self.sku_ids {
            let uuid = Uuid::parse_str(sku_id_str)
                .map_err(|_| format!("Invalid UUID format: {}", sku_id_str))?;
            sku_id_objects.push(SKUId::from_uuid(uuid));
        }

        Ok(FindVariantsQuery::new(sku_id_objects))
    }
}
