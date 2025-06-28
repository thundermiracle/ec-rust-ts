use crate::application::dto::VariantSummaryDTO;
use crate::presentation::variants::responses::{FindVariantsItemResponse, FindVariantsResponse};

pub struct FindVariantsPresenter;

impl FindVariantsPresenter {
    fn present_find_variants_single(dto: VariantSummaryDTO) -> FindVariantsItemResponse {
        FindVariantsItemResponse {
            sku_id: dto.sku_id.value().to_string(),
            price: dto.price,
            sale_price: dto.sale_price,
            image: dto.image,
            material: dto.material,
            dimensions: dto.dimensions,
        }
    }

    pub fn present(dtos: Vec<VariantSummaryDTO>) -> FindVariantsResponse {
        let variants = dtos.into_iter()
            .map(Self::present_find_variants_single)
            .collect();

        FindVariantsResponse { variants }
    }
} 