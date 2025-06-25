use std::sync::Arc;

use crate::application::repositories::ColorRepository;
use crate::application::dto::ColorListDTO;
use crate::application::error::ApplicationError;

pub struct GetColorListHandler {
    color_repository: Arc<dyn ColorRepository + Send + Sync>,
}

impl GetColorListHandler {
    pub fn new(color_repository: Arc<dyn ColorRepository + Send + Sync>) -> Self {
        Self { color_repository }
    }

    pub async fn handle(&self) -> Result<ColorListDTO, ApplicationError> {
        println!("->> get_color_list_handler");

        let color_list = self.color_repository.find_all().await?;

        Ok(color_list)
    }
}
