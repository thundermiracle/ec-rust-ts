/// ProductのViewModel（Application層）
/// CQRS Query側専用：Infrastructure層から直接構築
/// パフォーマンス重視でドメインモデルを経由しない
#[derive(Debug, Clone)]
pub struct ProductDTO {
    pub id: String,
    pub name: String,
    pub images: Vec<String>,
    pub category: String,
    pub description: String,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub variants: Vec<VariantDTO>,
}

/// Product variantのビューモデル（Application層）
#[derive(Debug, Clone)]
pub struct VariantDTO {
    pub id: String,
    pub sku_code: String,
    pub name: String,
    pub color: String,
    pub material: String,
    pub dimensions: String,
    pub price: u32,
    pub sale_price: Option<u32>,
    pub stock_quantity: u32,
    pub reserved_quantity: u32,
    pub display_order: u32,
    pub image: Option<String>,
    pub is_on_sale: bool,
    pub is_sold_out: bool,
}


impl ProductDTO {}

impl VariantDTO {
    /// Infrastructure層から直接構築用
    pub fn new(
        id: String,
        sku_code: String,
        name: String,
        color: String,
        material: String,
        dimensions: String,
        price: u32,
        sale_price: Option<u32>,
        stock_quantity: u32,
        reserved_quantity: u32,
        display_order: u32,
        image: Option<String>,
    ) -> Self {
        let is_on_sale = sale_price.is_some();
        let is_sold_out = stock_quantity == 0;

        Self {
            id,
            sku_code,
            name,
            color,
            material,
            dimensions,
            price,
            sale_price,
            stock_quantity,
            reserved_quantity,
            display_order,
            image,
            is_on_sale,
            is_sold_out,
        }
    }
}


