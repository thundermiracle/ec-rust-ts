use crate::domain::models::{ProductImage, ProductId, ImageId, ImageUrl, ImageAlt};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::ProductImageEntity;
use uuid::Uuid;

/// 製品画像マッパー - エンティティとドメインモデル間の変換を担当
pub struct ProductImageMapper;

impl ProductImageMapper {
    /// エンティティからドメインモデルへの変換
    pub fn to_domain(entity: &ProductImageEntity) -> Result<ProductImage, DomainError> {
        // 画像IDの変換
        let image_id = Uuid::parse_str(&entity.id)
            .map(ImageId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid image UUID".to_string()))?;
        
        // 製品IDの変換
        let product_id = Uuid::parse_str(&entity.product_id)
            .map(ProductId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid product UUID".to_string()))?;
        
        // 画像URL
        let url = ImageUrl::new(&entity.url)?;
        
        // Alt テキスト
        let alt = ImageAlt::new(&entity.alt_text);
        
        // 製品画像ドメインモデルの作成
        let mut image = ProductImage::new(
            image_id,
            product_id,
            url,
            alt,
        )?;
        
        // メイン画像フラグの設定
        if entity.is_main_image {
            image.set_as_main();
        }
        
        // 表示順序の設定
        image.set_display_order(entity.display_order);
        
        Ok(image)
    }
    
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(image: &ProductImage) -> ProductImageEntity {
        ProductImageEntity {
            id: image.id().to_string(),
            product_id: image.product_id().to_string(),
            url: image.url().value().to_string(),
            alt_text: image.alt().value().to_string(),
            is_main_image: image.is_main_image(),
            display_order: image.display_order(),
            created_at: image.created_at().to_rfc3339(),
            updated_at: image.updated_at().to_rfc3339(),
        }
    }
} 