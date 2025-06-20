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
        let regular_price = Money::from_string(&entity.regular_price)?;
        let sale_price = if !entity.sale_price.is_empty() {
            Some(Money::from_string(&entity.sale_price)?)
        } else {
            None
        };
        
        // バリエーション属性の構築
        // 注: VariantAttributes の正確な構造によって実装が異なる可能性あり
        let variant_attributes = if let Some(color) = color_entity {
            // カラー情報を含むバリエーション属性を構築
            VariantAttributes::new_with_color(
                &color.name, 
                &color.hex_code
            )?
        } else {
            VariantAttributes::default()
        };
        
        // SKUドメインモデルの作成
        SKU::new(
            sku_id,
            product_id,
            sku_code,
            regular_price,
            entity.stock_quantity,
            variant_attributes,
        ).map(|mut sku| {
            // 追加プロパティの設定
            if let Some(price) = sale_price {
                let _ = sku.set_sale_price(price);
            }
            
            if entity.is_active {
                let _ = sku.activate();
            } else {
                let _ = sku.discontinue();
            }
            
            sku
        })
    }
    
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(sku: &SKU) -> SKUEntity {
        SKUEntity {
            id: sku.id().to_string(),
            product_id: sku.product_id().to_string(),
            sku_code: sku.sku_code().value().to_string(),
            regular_price: sku.regular_price().to_string(),
            sale_price: sku.sale_price().map_or_else(String::new, |price| price.to_string()),
            stock_quantity: sku.available_quantity(),
            reserved_quantity: sku.reserved_quantity(),
            is_active: sku.is_active(),
            created_at: sku.created_at().to_rfc3339(),
            updated_at: sku.updated_at().to_rfc3339(),
        }
    }
} 