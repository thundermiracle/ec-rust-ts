use crate::application::dto::CreateOrderResultDTO;
use crate::presentation::orders::CreateOrderResponse;

pub struct OrderPresenter;

impl OrderPresenter {
    pub fn to_response(result: CreateOrderResultDTO) -> CreateOrderResponse {
        CreateOrderResponse {
            order_id: result.order_id,
            order_number: result.order_number,
            total_amount: result.total_amount,
            status: result.status,
        }
    }
}