use crate::domain::models::{ColorId, Color, HexCode};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::ColorEntity;
use uuid::Uuid;
use std::collections::HashMap;

/// カラーマッパー - カラーエンティティとドメインモデル間の変換を担当
pub struct ColorMapper;

impl ColorMapper {
    /// エンティティからドメインモデルへの変換
    pub fn to_domain(entity: &ColorEntity) -> Result<Color, DomainError> {
        // カラーIDの変換
        let color_id = Uuid::parse_str(&entity.id)
            .map(ColorId::from_uuid)
            .map_err(|_| DomainError::InvalidProductData("Invalid color UUID".to_string()))?;
        
        // HEXコードの生成
        let hex_code = HexCode::new(&entity.hex_code)?;
        
        // カラードメインモデルの作成
        Color::new(color_id, &entity.name, hex_code)
    }
    
    /// ドメインモデルからエンティティへの変換
    pub fn from_domain(color: &Color) -> ColorEntity {
        ColorEntity {
            id: color.id().to_string(),
            name: color.name().to_string(),
            hex_code: color.hex_code().value().to_string(),
            created_at: color.created_at().to_rfc3339(),
            updated_at: color.updated_at().to_rfc3339(),
        }
    }
    
    /// カラーのコレクションをマッピングし、IDによる参照を効率化
    pub fn build_color_map(colors: &[ColorEntity]) -> Result<HashMap<String, Color>, DomainError> {
        colors.iter()
            .map(|entity| {
                ColorMapper::to_domain(entity).map(|color| (entity.id.clone(), color))
            })
            .collect()
    }
} 