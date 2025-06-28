use crate::application::dto::ColorListDTO;
use crate::presentation::colors::responses::{GetColorListResponse, GetColorListItemResponse};

pub struct GetColorListPresenter;

impl GetColorListPresenter {
    pub fn present(color_list_dto: ColorListDTO) -> GetColorListResponse {
        GetColorListResponse {
            colors: color_list_dto.colors.into_iter().map(|color| GetColorListItemResponse {
                id: color.id as u32,
                name: color.name,
                hex: color.hex,
            }).collect(),
        }
    }
} 