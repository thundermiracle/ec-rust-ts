#[derive(Debug, Clone)]
pub struct ProductListViewModel {
    pub products: Vec<ProductSummaryViewModel>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub has_next_page: bool,
    pub has_previous_page: bool,
}

#[derive(Debug, Clone)]
pub struct ProductSummaryViewModel {
    pub id: String,
    pub name: String,
    pub category: String,
    pub base_price: u32,
    pub sale_price: Option<u32>,
    pub image: Option<String>,
    pub colors: Vec<String>,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub stock_quantity: u32,
}

impl ProductSummaryViewModel {
    pub fn new(
        id: String,
        name: String,
        category: String,
        base_price: u32,
        sale_price: Option<u32>,
        image: Option<String>,
        colors: Vec<String>,
        is_best_seller: bool,
        is_quick_ship: bool,
        stock_quantity: u32,
    ) -> Self {
        Self {
            id,
            name,
            category,
            base_price,
            sale_price,
            image,
            colors,
            is_best_seller,
            is_quick_ship,
            stock_quantity,
        }
    }

    pub fn is_on_sale(&self) -> bool {
        self.sale_price.is_some()
    }

    pub fn is_sold_out(&self) -> bool {
        self.stock_quantity == 0
    }
}