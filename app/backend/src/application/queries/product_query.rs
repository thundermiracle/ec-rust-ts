use crate::domain::models::Product;
use thiserror::Error;

use super::{
    CategoryQuery, ImageQuery, PriceQuery, StatusQuery, StockQuery, VariantQuery,
};

/// 商品クエリ処理中に発生するエラー
#[derive(Debug, Error)]
pub enum QueryError {
    #[error("ドメインモデルからクエリへの変換に失敗しました: {0}")]
    MappingError(String),
}

/// 商品クエリ結果 - API応答に最適化されたDTO
/// Clean Architecture: Application層のQuery DTO.
/// Presentation層に渡すための、ビジネスロジックを含まない純粋なデータ構造です。
#[derive(Debug, Clone)]
pub struct ProductQuery {
    // 基本情報
    pub id: u32,
    pub name: String,
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,

    // 構造化された情報
    pub price: PriceQuery,
    pub category: CategoryQuery,
    pub stock: StockQuery,
    pub images: ImageQuery,
    pub status: StatusQuery,

    // 関連情報
    pub colors: Vec<String>,
    pub tags: Vec<String>,
    pub variants: Vec<VariantQuery>,
}

/// ドメインモデルからクエリDTOへの変換を担当するマッパー
pub struct ProductQueryMapper;

impl ProductQueryMapper {
    /// ProductドメインモデルをProductQuery DTOに変換します
    pub fn from_domain(product: Product) -> Result<ProductQuery, QueryError> {
        let variants: Vec<VariantQuery> = product
            .variants
            .iter()
            .map(|variant| VariantQuery {
                id: variant.id.value().to_string(),
                name: variant.name.clone(),
                price: variant.current_price().yen(),
                color: variant.color.clone(),
                image: variant.image_url.clone(),
                is_available: variant.is_available,
            })
            .collect();

        // TODO: product.category.id がドメインモデルに存在することを前提としています。
        // ドメインモデルに `id` がない場合は、ここで解決する必要があります。
        let category_id = product.category.id.value().to_string();

        let product_query = ProductQuery {
            id: product.id,
            name: product.name.clone(),
            description: product.description.clone(),
            material: product.material.clone(),
            dimensions: product.dimensions.clone(),
            price: PriceQuery {
                base_price: product.base_price.yen(),
                sale_price: product.sale_price.map(|p| p.yen()),
                current_price: product.current_price().yen(),
                discount_percentage: product.discount_percentage(),
                savings_amount: product.savings_amount().yen(),
            },
            category: CategoryQuery {
                id: category_id,
                name: product.category.name.clone(),
                slug: product.category.slug.clone(),
            },
            stock: StockQuery {
                quantity: product.quantity,
                is_sold_out: product.is_sold_out(),
                is_available: product.is_available_for_purchase(),
            },
            images: ImageQuery {
                images: product.image_urls(),
                main_image: product.main_image().map(|img| img.url().to_string()),
            },
            status: StatusQuery {
                is_on_sale: product.is_on_sale,
                is_best_seller: product.is_best_seller,
                is_quick_ship: product.is_quick_ship,
                is_active: product.is_available,
            },
            colors: product.color_names(),
            tags: product.tag_names(),
            variants,
        };

        Ok(product_query)
    }
}

/// ProductQueryを構築するためのビルダー
/// # Example
/// ```
/// let query = ProductQueryBuilder::new()
///     .id(1)
///     .name("Product Name".to_string())
///     // ... 他のフィールドを設定 ...
///     .build()?;
/// ```
#[derive(Default)]
pub struct ProductQueryBuilder {
    id: Option<u32>,
    name: Option<String>,
    description: Option<String>,
    material: Option<String>,
    dimensions: Option<String>,
    price: Option<PriceQuery>,
    category: Option<CategoryQuery>,
    stock: Option<StockQuery>,
    images: Option<ImageQuery>,
    status: Option<StatusQuery>,
    colors: Vec<String>,
    tags: Vec<String>,
    variants: Vec<VariantQuery>,
}

#[derive(Debug, Error)]
pub enum BuildError {
    #[error("必須フィールドが不足しています: {0}")]
    MissingField(String),
}

impl ProductQueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn id(mut self, id: u32) -> Self {
        self.id = Some(id);
        self
    }
    
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    // 他のセッターメソッドも同様に実装...

    pub fn build(self) -> Result<ProductQuery, BuildError> {
        let id = self.id.ok_or_else(|| BuildError::MissingField("id".to_string()))?;
        let name = self.name.ok_or_else(|| BuildError::MissingField("name".to_string()))?;
        // 他の必須フィールドのチェック...

        Ok(ProductQuery {
            id,
            name,
            description: self.description.unwrap_or_default(),
            material: self.material,
            dimensions: self.dimensions,
            price: self.price.ok_or_else(|| BuildError::MissingField("price".to_string()))?,
            category: self.category.ok_or_else(|| BuildError::MissingField("category".to_string()))?,
            stock: self.stock.ok_or_else(|| BuildError::MissingField("stock".to_string()))?,
            images: self.images.ok_or_else(|| BuildError::MissingField("images".to_string()))?,
            status: self.status.ok_or_else(|| BuildError::MissingField("status".to_string()))?,
            colors: self.colors,
            tags: self.tags,
            variants: self.variants,
        })
    }
}