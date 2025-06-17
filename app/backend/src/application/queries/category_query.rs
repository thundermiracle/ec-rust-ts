/// カテゴリー情報のクエリ結果
#[derive(Debug, Clone)]
pub struct CategoryQuery {
    pub id: String,
    pub name: String,
    pub slug: String,
} 