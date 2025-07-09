pub struct Address {
    pub postal_code: String,
    pub prefecture: String,
    pub city: String,
    pub street: String,
    pub building: Option<String>,
    pub phone_number: String,
}

impl Address {
    pub fn new(postal_code: String, prefecture: String, city: String, street: String, building: Option<String>, phone_number: String) -> Self {
        Self { postal_code, prefecture, city, street, building, phone_number }
    }
}
