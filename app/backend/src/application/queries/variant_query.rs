/// 商品バリアントのクエリ結果
/// Clean Architecture: Application層のQuery DTO
#[derive(Debug, Clone)]
pub struct VariantQuery {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    pub image: Option<String>,
    pub is_available: bool,
} 