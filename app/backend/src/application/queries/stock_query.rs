/// 在庫情報のクエリ結果
#[derive(Debug, Clone)]
pub struct StockQuery {
    pub quantity: u32,
    pub is_sold_out: bool,
    pub is_available: bool,
} 