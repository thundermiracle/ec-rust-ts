/// 商品ステータスフラグのクエリ結果
#[derive(Debug, Clone)]
pub struct StatusQuery {
    pub is_on_sale: bool,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub is_active: bool,
} 