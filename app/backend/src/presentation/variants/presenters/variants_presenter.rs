use crate::application::dto::VariantInfoDTO;
use crate::presentation::variants::responses::{VariantResponse, VariantsResponse};

pub struct VariantsPresenter;

impl VariantsPresenter {
    fn present_single(dto: VariantInfoDTO) -> VariantResponse {
        VariantResponse {
            sku_id: dto.sku_id.value().to_string(),
            price: dto.price,
            sale_price: dto.sale_price,
            image: dto.image,
            material: dto.material,
            dimensions: dto.dimensions,
        }
    }

    pub fn present(dtos: Vec<VariantInfoDTO>) -> VariantsResponse {
        let variants = dtos.into_iter()
            .map(Self::present_single)
            .collect();

        VariantsResponse { variants }
    }
} 