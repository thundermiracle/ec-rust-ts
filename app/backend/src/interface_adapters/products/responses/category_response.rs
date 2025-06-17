use serde::Serialize;

/// カテゴリー応答構造体（将来のために残しておく）
#[derive(Debug, Serialize)]
pub struct CategoryResponse {
    pub id: String,                    // 文字列IDに変更
    pub name: String,
    pub slug: String,
} 