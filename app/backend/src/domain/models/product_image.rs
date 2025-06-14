use crate::domain::error::DomainError;

/// 商品画像ドメインモデル
/// 商品に関連付けられた画像を管理し、表示順序をサポート
#[derive(Debug, Clone, PartialEq)]
pub struct ProductImage {
    pub id: ProductImageId,
    pub product_id: ProductImageProductId,
    pub image_url: ImageUrl,
    pub sort_order: u32,
}

/// 商品画像ID値オブジェクト
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductImageId(u32);

/// 商品ID参照値オブジェクト（ProductImageが参照する商品ID）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductImageProductId(String);

/// 画像URL値オブジェクト
#[derive(Debug, Clone, PartialEq)]
pub struct ImageUrl(String);

impl ProductImage {
    /// 新しい商品画像を作成
    pub fn new(
        id: ProductImageId,
        product_id: ProductImageProductId,
        image_url: ImageUrl,
        sort_order: u32,
    ) -> Result<Self, DomainError> {
        // ソート順序のビジネスルール: 最大値制限
        if sort_order > 9999 {
            return Err(DomainError::InvalidProductData(
                "Sort order cannot exceed 9999".to_string(),
            ));
        }

        Ok(Self {
            id,
            product_id,
            image_url,
            sort_order,
        })
    }

    /// メイン画像かどうかを判定（ソート順0番）
    pub fn is_main_image(&self) -> bool {
        self.sort_order == 0
    }

    /// 画像URLを取得
    pub fn url(&self) -> &str {
        self.image_url.value()
    }

    /// 商品IDを取得
    pub fn product_id(&self) -> &ProductImageProductId {
        &self.product_id
    }
}

impl ProductImageId {
    /// 新しい商品画像IDを作成
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    /// IDの値を取得
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl ProductImageProductId {
    /// 新しい商品ID参照を作成
    pub fn new(product_id: String) -> Result<Self, DomainError> {
        if product_id.trim().is_empty() {
            return Err(DomainError::InvalidProductData(
                "Product ID cannot be empty".to_string(),
            ));
        }
        Ok(Self(product_id))
    }

    /// 商品IDの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl ImageUrl {
    /// 新しい画像URLを作成
    pub fn new(url: String) -> Result<Self, DomainError> {
        let trimmed = url.trim();
        
        if trimmed.is_empty() {
            return Err(DomainError::InvalidProductData(
                "Image URL cannot be empty".to_string(),
            ));
        }

        // 基本的なURL形式チェック
        if !Self::is_valid_url(trimmed) {
            return Err(DomainError::InvalidProductData(
                "Invalid image URL format".to_string(),
            ));
        }

        Ok(Self(trimmed.to_string()))
    }

    /// URLの値を取得
    pub fn value(&self) -> &str {
        &self.0
    }

    /// 基本的なURL形式バリデーション
    fn is_valid_url(url: &str) -> bool {
        // HTTPまたはHTTPSで始まる、または相対パス
        url.starts_with("http://") 
            || url.starts_with("https://") 
            || url.starts_with("/")
            || url.starts_with("./")
    }
}

impl std::fmt::Display for ProductImageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ProductImageProductId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_product_image() {
        let id = ProductImageId::new(1);
        let product_id = ProductImageProductId::new("desk-walnut-1".to_string()).unwrap();
        let image_url = ImageUrl::new("https://example.com/image.jpg".to_string()).unwrap();
        
        let product_image = ProductImage::new(id, product_id, image_url, 0);
        assert!(product_image.is_ok());
        
        let product_image = product_image.unwrap();
        assert!(product_image.is_main_image());
    }

    #[test]
    fn create_secondary_image() {
        let id = ProductImageId::new(2);
        let product_id = ProductImageProductId::new("desk-walnut-1".to_string()).unwrap();
        let image_url = ImageUrl::new("https://example.com/image2.jpg".to_string()).unwrap();
        
        let product_image = ProductImage::new(id, product_id, image_url, 1);
        assert!(product_image.is_ok());
        
        let product_image = product_image.unwrap();
        assert!(!product_image.is_main_image());
    }

    #[test]
    fn reject_invalid_sort_order() {
        let id = ProductImageId::new(1);
        let product_id = ProductImageProductId::new("desk-walnut-1".to_string()).unwrap();
        let image_url = ImageUrl::new("https://example.com/image.jpg".to_string()).unwrap();
        
        let product_image = ProductImage::new(id, product_id, image_url, 10000);
        assert!(product_image.is_err());
    }

    #[test]
    fn accept_valid_urls() {
        assert!(ImageUrl::new("https://example.com/image.jpg".to_string()).is_ok());
        assert!(ImageUrl::new("http://example.com/image.png".to_string()).is_ok());
        assert!(ImageUrl::new("/images/product.jpg".to_string()).is_ok());
        assert!(ImageUrl::new("./assets/image.jpg".to_string()).is_ok());
    }

    #[test]
    fn reject_invalid_urls() {
        assert!(ImageUrl::new("".to_string()).is_err());
        assert!(ImageUrl::new("invalid-url".to_string()).is_err());
        assert!(ImageUrl::new("ftp://example.com/image.jpg".to_string()).is_err());
    }

    #[test]
    fn reject_empty_product_id() {
        let product_id = ProductImageProductId::new("".to_string());
        assert!(product_id.is_err());
    }
} 