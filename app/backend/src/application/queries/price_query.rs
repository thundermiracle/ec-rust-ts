/// 価格情報のクエリ結果
#[derive(Debug, Clone)]
pub struct PriceQuery {
    pub base_price: u32,
    pub sale_price: Option<u32>,
    pub current_price: u32,
    pub discount_percentage: Option<u8>,
    pub savings_amount: u32,
} 