use crate::domain::models::Product;

/// GetProductDetail Query Result
/// Clean Architecture: Application層のQuery結果DTO
/// UseCaseが返す構造化されたデータを表現
#[derive(Debug, Clone)]
pub struct GetProductDetailQuery {
    pub product: Product,
}

impl GetProductDetailQuery {
    /// 新しいクエリ結果を作成
    pub fn new(product: Product) -> Self {
        Self { product }
    }

    /// 商品IDを取得
    pub fn product_id(&self) -> u32 {
        self.product.id
    }

    /// 商品が購入可能かどうかを判定
    pub fn is_available(&self) -> bool {
        self.product.is_available_for_purchase()
    }

    /// メイン画像URLを取得
    pub fn main_image_url(&self) -> Option<String> {
        self.product.images.first().map(|img| img.url().to_string())
    }

    /// 全画像URLを取得
    pub fn image_urls(&self) -> Vec<String> {
        self.product.images
            .iter()
            .map(|img| img.url().to_string())
            .collect()
    }

    /// 色名一覧を取得
    pub fn color_names(&self) -> Vec<String> {
        self.product.colors
            .iter()
            .map(|color| color.name().value().to_string())
            .collect()
    }

    /// タグ名一覧を取得
    pub fn tag_names(&self) -> Vec<String> {
        self.product.tags
            .iter()
            .map(|tag| tag.name().to_string())
            .collect()
    }
} 