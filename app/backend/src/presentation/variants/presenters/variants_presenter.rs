use crate::application::dto::VariantSummaryDTO;
use crate::presentation::variants::responses::{VariantListItemResponse, VariantListResponse};

pub struct VariantsPresenter;

impl VariantsPresenter {
    fn present_single(dto: VariantSummaryDTO) -> VariantListItemResponse {
        VariantListItemResponse {
            sku_id: dto.sku_id.value().to_string(),
            price: dto.price,
            sale_price: dto.sale_price,
            image: dto.image,
            material: dto.material,
            dimensions: dto.dimensions,
        }
    }

    pub fn present(dtos: Vec<VariantSummaryDTO>) -> VariantListResponse {
        let variants = dtos.into_iter()
            .map(Self::present_single)
            .collect();

        VariantListResponse { variants }
    }
} 