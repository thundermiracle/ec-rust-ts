use crate::domain::models::{Tag, TagId, TagName, TagSlug};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::{TagEntity, ProductTagEntity};
use uuid::Uuid;
use std::collections::HashMap;

/// タグマッパー - エンティティとドメインモデル間の変換を担当
pub struct TagMapper;

impl TagMapper {
    /// エンティティからドメインモデルへの変換
    pub fn to_domain(tag_entity: &TagEntity) -> Result<Tag, DomainError> {
        // タグIDの変換
        let tag_id = Uuid::parse_str(&tag_entity.id)
            .map(TagId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid tag UUID".to_string()))?;
        
        // タグ名とスラッグを生成
        let name = TagName::new(&tag_entity.name)?;
        let slug = TagSlug::new(&tag_entity.slug)?;
        
        // タグドメインモデルの作成
        Tag::new(tag_id, name, slug)
    }
    
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(tag: &Tag) -> TagEntity {
        TagEntity {
            id: tag.id().to_string(),
            name: tag.name().value().to_string(),
            slug: tag.slug().value().to_string(),
            created_at: tag.created_at().to_rfc3339(),
            updated_at: tag.updated_at().to_rfc3339(),
        }
    }
    
    /// 製品タグエンティティの作成（製品とタグの関連付け）
    pub fn create_product_tag_entity(product_id: &str, tag_id: &str) -> ProductTagEntity {
        ProductTagEntity {
            product_id: product_id.to_string(),
            tag_id: tag_id.to_string(),
        }
    }
    
    /// 製品に関連するタグのコレクションをマッピング
    pub fn map_product_tags(
        product_tags: &[ProductTagEntity],
        tag_details: &[TagEntity]
    ) -> Result<Vec<Tag>, DomainError> {
        // タグIDからTagEntityへのマップを作成して検索を効率化
        let tag_map: HashMap<String, &TagEntity> = tag_details
            .iter()
            .map(|tag| (tag.id.clone(), tag))
            .collect();
        
        // 製品に関連するタグをマッピング
        product_tags
            .iter()
            .filter_map(|pt| tag_map.get(&pt.tag_id))
            .map(|tag_entity| TagMapper::to_domain(tag_entity))
            .collect()
    }
} 