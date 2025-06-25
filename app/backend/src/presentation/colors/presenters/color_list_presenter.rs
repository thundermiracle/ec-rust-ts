use crate::application::dto::ColorListDTO;
use crate::presentation::colors::responses::{ColorListResponse, ColorListItemResponse};

pub struct ColorListPresenter;

impl ColorListPresenter {
    pub fn present(color_list_dto: ColorListDTO) -> ColorListResponse {
        ColorListResponse {
            colors: color_list_dto.colors.into_iter().map(|color| ColorListItemResponse {
                id: color.id as u32,
                name: color.name,
                hex: color.hex,
            }).collect(),
        }
    }
}