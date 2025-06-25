pub struct ColorListDTO {
    pub colors: Vec<ColorDTO>,
}

impl ColorListDTO {
    pub fn new(colors: Vec<ColorDTO>) -> Self {
        Self { colors }
    }
}

pub struct ColorDTO {
    pub id: i64,
    pub name: String,
    pub hex: String,
}
