use crate::domain::models::{Product, ProductId, CategoryId, ProductName, Description};
use crate::domain::error::DomainError;
use crate::frameworks_and_drivers::persistence::entities::ProductEntity;
use uuid::Uuid;

/// ドメインモデル変換用のマッパー
/// Clean Architecture: Frameworks & Drivers層からDomain層への変換を担当
pub struct ProductMapper;

impl ProductMapper {
    /// エンティティからドメインモデルへの変換
    /// 集約データからProductドメインモデルを構築
    pub fn to_domain(data: ProductAggregateData) -> Result<Product, DomainError> {
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
        let mut product = Product::create(
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
        
        // 注: ここでSKU、画像、タグなどの関連オブジェクトを追加する処理を実装
        // 別途SKUMapper、ImageMapper、TagMapperなどを作成して利用すると良い
        
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
} 