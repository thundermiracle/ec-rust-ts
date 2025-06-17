/// 画像情報のクエリ結果
#[derive(Debug, Clone)]
pub struct ImageQuery {
    pub images: Vec<String>,
    pub main_image: Option<String>,
} 