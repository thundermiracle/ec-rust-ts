use crate::domain::models::{SKU, SKUId, ProductId, SKUCode, Money, VariantAttributes};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::{SKUEntity, ColorEntity};
use uuid::Uuid;

/// SKUマッパー - SKUエンティティとドメインモデル間の変換を担当
pub struct SKUMapper;

impl SKUMapper {
    /// エンティティからドメインモデルへの変換
    pub fn to_domain(
        entity: &SKUEntity, 
        color_entity: Option<&ColorEntity>
    ) -> Result<SKU, DomainError> {
        // SKU IDの変換
        let sku_id = Uuid::parse_str(&entity.id)
            .map(SKUId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid SKU UUID".to_string()))?;
        
        // 関連する製品IDの変換
        let product_id = Uuid::parse_str(&entity.product_id)
            .map(ProductId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid product UUID".to_string()))?;
        
        // SKUコードの生成
        let sku_code = SKUCode::new(&entity.sku_code)?;
        
        // 価格の変換
        let base_price = Money::from_yen(entity.base_price as i32);
        let sale_price = entity.sale_price.map(|price| Money::from_yen(price as i32));
        
        // バリエーション属性の構築
        let mut variant_attributes = VariantAttributes::new();
        
        if let Some(color) = color_entity {
            variant_attributes.set_color(&color.name, &color.hex)?;
        }
        
        if let Some(dimensions) = &entity.dimensions {
            variant_attributes.set_dimensions(dimensions)?;
        }
        
        if let Some(material) = &entity.material {
            variant_attributes.set_material(material)?;
        }
        
        // SKUドメインモデルの作成
        let mut sku = SKU::create_with_variants(
            sku_id,
            product_id,
            sku_code,
            crate::domain::models::SKUName::new(&entity.name)?,
            variant_attributes,
            base_price,
            entity.stock_quantity as u32,
        )?;
        
        // 追加プロパティの設定
        if let Some(price) = sale_price {
            sku.set_sale_price(price)?;
        }
        
        sku.set_display_order(entity.display_order as u32);
        
        Ok(sku)
    }
    
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(sku: &SKU) -> SKUEntity {
        SKUEntity {
            id: sku.id().to_string(),
            product_id: sku.product_id().to_string(),
            sku_code: sku.sku_code().value().to_string(),
            name: sku.name().value().to_string(),
            color_id: None, // This should be set by the calling code with color information
            dimensions: sku.variant_attributes().dimensions().map(|d| d.to_string()),
            material: sku.variant_attributes().material().map(|m| m.to_string()),
            base_price: sku.base_price().yen() as i64,
            sale_price: sku.sale_price().map(|price| price.yen() as i64),
            stock_quantity: sku.total_quantity() as i64,
            reserved_quantity: sku.reserved_quantity() as i64,
            low_stock_threshold: Some(5), // Default threshold
            display_order: sku.display_order() as i64,
            image_url: None, // This should be set by the calling code
            created_at: sku.created_at().to_rfc3339(),
            updated_at: sku.updated_at().to_rfc3339(),
        }
    }
} 