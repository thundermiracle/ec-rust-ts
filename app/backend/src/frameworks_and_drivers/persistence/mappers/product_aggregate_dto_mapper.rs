use crate::domain::models::{Product, ProductId, CategoryId, ProductName, Description};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::{
    ProductEntity,
    SKUEntity,
    ProductImageEntity,
    ProductTagEntity,
    TagEntity,
    ColorEntity,
};
use uuid::Uuid;
use std::collections::HashMap;
use super::{
    sku_mapper::SKUMapper,
    product_image_mapper::ProductImageMapper,
    tag_mapper::TagMapper,
    color_mapper::ColorMapper,
    product_aggregate_dto::ProductAggregateDto,
};

/// ドメインモデル変換用のマッパー
/// Clean Architecture: Frameworks & Drivers層からDomain層への変換を担当
pub struct ProductAggregateDtoMapper;

impl ProductAggregateDtoMapper {
    /// エンティティからドメインモデルへの変換
    /// 集約データからProductドメインモデルを構築
    pub fn to_domain(data: ProductAggregateDto) -> Result<Product, DomainError> {
        // 製品IDの変換
        let product_id = Uuid::parse_str(&data.product.id)
            .map(ProductId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid product UUID".to_string()))?;
        
        // 値オブジェクトの生成
        let product_name = ProductName::new(data.product.name)?;
        let description = Description::new(data.product.description);
        
        // カテゴリIDの変換
        let category_id = CategoryId::new(
            data.product.category_id.parse::<u32>()
                .map_err(|_| DomainError::InvalidProductData("Invalid category ID".to_string()))?
        )?;
        
        // ドメインモデルを作成
        let mut product = Product::new(
            product_id,
            product_name,
            description,
            category_id,
        )?;
        
        // ステータスフラグの設定
        if data.product.is_best_seller {
            product.mark_as_best_seller();
        }
        
        if data.product.is_quick_ship {
            product.enable_quick_ship();
        }
        
        // カラーの変換（効率的な検索のためにハッシュマップを作成）
        let color_map: HashMap<String, &ColorEntity> = data.colors
            .iter()
            .map(|c| (c.id.clone(), c))
            .collect();
        
        // SKUの追加
        for sku_entity in data.skus {
            // SKUに対応するカラーエンティティを検索
            let color_entity = sku_entity
                .color_id
                .as_ref()
                .and_then(|color_id| color_map.get(color_id));
            
            let sku = SKUMapper::to_domain(&sku_entity, color_entity)?;
            product.add_sku(sku)?;
        }
        
        // 画像の追加
        for image_entity in &data.images {
            let product_image = ProductImageMapper::to_domain(image_entity)?;
            product.add_image(product_image);
        }
        
        // タグの追加
        let tags = TagMapper::map_product_tags(&data.tags, &data.tag_details)?;
        for tag in tags {
            product.add_tag(tag);
        }
        
        Ok(product)
    }
    
    /// ドメインモデルから基本製品エンティティへの変換
    /// 注: これは基本情報のみを変換する。SKUなどは別のマッパーで処理すべき
    pub fn from_domain_basic(product: &Product) -> ProductEntity {
        ProductEntity {
            id: product.id().to_string(),
            name: product.name().value().to_string(),
            description: product.description().value().to_string(),
            category_id: product.category_id().value().to_string(),
            is_best_seller: product.is_best_seller(),
            is_quick_ship: product.is_quick_ship(),
            is_active: product.is_active(),
            created_at: product.created_at().to_rfc3339(),
            updated_at: product.updated_at().to_rfc3339(),
        }
    }
    
    /// ドメインモデルからエンティティ集約への変換（完全な変換）
    pub fn from_domain_aggregate(product: &Product) -> ProductAggregateDto {
        // 基本情報の変換
        let product_entity = ProductMapper::from_domain_basic(product);
        
        // SKUの変換
        let skus = product.skus()
            .iter()
            .map(|sku| SKUMapper::from_domain(sku))
            .collect();
        
        // 画像の変換
        let images = product.images()
            .iter()
            .map(|image| ProductImageMapper::from_domain(image))
            .collect();
        
        // タグとタグ関連の変換
        let mut tag_entities = Vec::new();
        let mut product_tag_entities = Vec::new();
        
        for tag in product.tags() {
            let tag_entity = TagMapper::from_domain(tag);
            let product_tag_entity = TagMapper::create_product_tag_entity(
                &product.id().to_string(), 
                &tag.id().to_string()
            );
            
            tag_entities.push(tag_entity);
            product_tag_entities.push(product_tag_entity);
        }
        
        // TODO: カラーの変換（現状では製品から直接カラーへのアクセスがないためスキップ）
        // 実際の実装では、SKUから関連するカラーを取得するロジックが必要
        
        ProductAggregateDto {
            product: product_entity,
            skus,
            images,
            tags: product_tag_entities,
            tag_details: tag_entities,
            colors: Vec::new(), // 上記のTODOにより、現状では空のリストを返す
        }
    }
} 