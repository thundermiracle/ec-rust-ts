use axum::Router;
use std::sync::Arc;

use crate::infrastructure::Container;
use crate::presentation::payment_methods::controllers::GetPaymentMethodListController;

pub fn routes() -> Router<Arc<Container>> {
    Router::new().merge(GetPaymentMethodListController::routes())
}
