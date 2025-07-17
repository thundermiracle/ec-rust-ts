#[derive(Debug, Clone)]
pub struct PaymentMethodListDTO {
    pub items: Vec<PaymentMethodDTO>,
}

#[derive(Debug, Clone)]
pub struct PaymentMethodDTO {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl PaymentMethodListDTO {
    pub fn new(items: Vec<PaymentMethodDTO>) -> Self {
        Self { items }
    }
}
