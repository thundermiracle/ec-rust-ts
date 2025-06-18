use crate::domain::models::{Product, Category, Color, ProductImage, Tag, CategoryId, ColorName, ProductImageId, ProductImageProductId, ImageUrl, TagSlug, Money, ProductVariant, ProductVariantId, ProductVariantProductId};
use crate::frameworks_and_drivers::database::db::get_db;
use crate::frameworks_and_drivers::persistence::entities::ProductEntity;
use crate::application::repositories::ProductRepository;
use crate::application::error::RepositoryError;
use crate::domain::error::DomainError;
use sqlx::FromRow;

/// 完全な商品情報を取得するためのJOIN結果構造体
#[derive(Debug, FromRow)]
struct CompleteProductRow {
    // Product fields
    id: i64,
    name: String,
    description: String,
    material: Option<String>,
    dimensions: Option<String>,
    base_price: i64,
    sale_price: Option<i64>,
    category_id: i64,
    color_id: Option<i64>,
    quantity: i64,
    reserved_quantity: i64,
    low_stock_threshold: Option<i64>,
    has_variants: bool,
    is_active: bool,
    is_best_seller: bool,
    is_quick_ship: bool,
    created_at: String,
    updated_at: String,
    
    // Category fields (nullable for LEFT JOIN)
    category_name: Option<String>,
    category_slug: Option<String>,
    category_parent_id: Option<i64>,
}

/// カラー情報取得用の構造体
#[derive(Debug, FromRow)]
struct ColorRow {
    color_name: String,
    hex: Option<String>,
}

/// 画像情報取得用の構造体
#[derive(Debug, FromRow)]
struct ImageRow {
    image_id: i64,
    image_url: String,
    sort_order: i64,
}

/// タグ情報取得用の構造体
#[derive(Debug, FromRow)]
struct TagRow {
    tag_name: String,
    tag_slug: String,
    color_code: Option<String>,
    priority: i64,
    is_system: bool,
}

/// バリアント情報取得用の構造体
#[derive(Debug, FromRow)]
struct VariantRow {
    id: i64,
    product_id: i64,
    name: String,
    base_price: i64,
    sale_price: Option<i64>,
    color_id: i64,
    color_name: String,
    color_hex: String,
    size: Option<String>,
    image_url: Option<String>,
    is_available: bool,
    stock_quantity: i64,
    reserved_quantity: i64,
    low_stock_threshold: Option<i64>,
    cost_price: Option<i64>,
}

/// SQLite商品リポジトリ実装 - 新しい正規化スキーマ対応
/// Clean Architecture: Frameworks & Drivers層のデータアクセス実装
pub struct SqliteProductRepository;

impl SqliteProductRepository {
    pub fn new() -> Self {
        Self {}
    }
    
    /// 関連データを含む完全なProductを構築
    async fn build_complete_product(
        &self,
        product_row: CompleteProductRow,
        color: Option<Color>,
        images: Vec<ProductImage>,
        tags: Vec<Tag>,
        variants: Vec<ProductVariant>,
    ) -> Result<Product, RepositoryError> {
        // Category構築 - new()メソッドを使用してプライベートフィールドにアクセス
        let category = if let (Some(name), Some(slug)) = (product_row.category_name, product_row.category_slug) {
            let category_id = CategoryId::new(product_row.category_id.to_string())
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            let parent_id = if let Some(parent_id_value) = product_row.category_parent_id {
                Some(CategoryId::new(parent_id_value.to_string())
                    .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?)
            } else {
                None
            };
            Category::new(category_id, name, slug, parent_id)
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?
        } else {
            Category::default()
        };

        // Product構築
        Product::new(
            product_row.id as u32,
            product_row.name,
            product_row.description,
            product_row.quantity as u32,
            product_row.reserved_quantity as u32,
            Money::from_yen(product_row.base_price as u32),
            category,
            color,
            product_row.has_variants,
            images,
            tags,
            variants,
            product_row.low_stock_threshold.map(|t| t as u32),
        ).map_err(|e| RepositoryError::QueryExecution(e.to_string()))
    }
}

#[async_trait::async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let entities = sqlx::query_as::<_, ProductEntity>(
            r#"
            SELECT id, name, description, material, dimensions, 
                   color_id, base_price, sale_price, category_id, 
                   stock_quantity, reserved_quantity, low_stock_threshold,
                   has_variants, is_active, is_best_seller, is_quick_ship,
                   created_at, updated_at
            FROM products 
            WHERE is_active = TRUE
            ORDER BY id
            "#
        )
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        let products = entities
            .into_iter()
            .map(|entity| entity.to_domain())
            .collect::<Result<Vec<Product>, _>>()
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        Ok(products)
    }

    async fn find_by_id(&self, id: u32) -> Result<Option<Product>, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        // メイン商品情報とカテゴリー情報を取得
        let product_row = sqlx::query_as::<_, CompleteProductRow>(
            r#"
            SELECT 
                p.id, p.name, p.description, p.material, p.dimensions,
                p.base_price, p.sale_price, p.category_id, p.color_id,
                p.stock_quantity as quantity, /* stock_quantityをquantityにエイリアス */
                p.reserved_quantity, p.low_stock_threshold,
                p.has_variants, p.is_active, p.is_best_seller, p.is_quick_ship,
                p.created_at, p.updated_at,
                c.name as category_name,
                c.slug as category_slug,
                c.parent_id as category_parent_id
            FROM products p
            LEFT JOIN categories c ON p.category_id = c.id
            WHERE p.id = ? AND p.is_active = TRUE
            "#
        )
        .bind(id as i64)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let product_row = match product_row {
            Some(row) => row,
            None => return Ok(None),
        };

        // カラー情報を取得（単一商品の場合）
        let color = if !product_row.has_variants {
            if let Some(color_id) = product_row.color_id {
                let color_row = sqlx::query_as::<_, (String, String)>(
                    r#"
                    SELECT name, hex FROM colors WHERE id = ?
                    "#
                )
                .bind(color_id)
                .fetch_optional(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

                if let Some((color_name, hex_code)) = color_row {
                    let color_name = ColorName::new(color_name)
                        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
                    let color = Color::new(color_id as u32, color_name, hex_code, None, None)
                        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
                    Some(color)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        // 画像情報を取得
        let image_rows = sqlx::query_as::<_, ImageRow>(
            r#"
            SELECT id as image_id, image_url, sort_order
            FROM product_images
            WHERE product_id = ?
            ORDER BY sort_order
            "#
        )
        .bind(id as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let images: Result<Vec<ProductImage>, DomainError> = image_rows
            .into_iter()
            .map(|row| {
                let image_id = ProductImageId::new(row.image_id as u32);
                let product_image_id = ProductImageProductId::new(id.to_string())?;
                let image_url = ImageUrl::new(row.image_url)?;
                ProductImage::new(image_id, product_image_id, image_url, row.sort_order as u32)
            })
            .collect();
        let images = images.map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // タグ情報を取得
        let tag_rows = sqlx::query_as::<_, TagRow>(
            r#"
            SELECT t.name as tag_name, t.slug as tag_slug, t.color_code, t.priority, t.is_system
            FROM product_tags pt
            JOIN tags t ON pt.tag_id = t.id
            WHERE pt.product_id = ?
            ORDER BY t.priority DESC, t.name
            "#
        )
        .bind(id as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let tags: Result<Vec<Tag>, DomainError> = tag_rows
            .into_iter()
            .map(|row| {
                let tag_slug = TagSlug::new(row.tag_slug)?;
                Tag::new(
                    tag_slug,
                    row.tag_name,
                    row.color_code,
                    row.priority as u8,
                    row.is_system,
                )
            })
            .collect();
        let tags = tags.map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // バリアント情報を取得
        let variant_rows = sqlx::query_as::<_, VariantRow>(
            r#"
            SELECT 
                pv.id, pv.product_id, pv.name, pv.base_price, pv.sale_price, 
                c.id as color_id, c.name as color_name, c.hex as color_hex,
                pv.size, pv.image_url, pv.is_available, pv.stock_quantity, pv.reserved_quantity,
                pv.low_stock_threshold, pv.cost_price
            FROM product_variants pv
            JOIN colors c ON pv.color_id = c.id
            WHERE pv.product_id = ?
            ORDER BY pv.id
            "#
        )
        .bind(id as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let variants: Result<Vec<ProductVariant>, DomainError> = variant_rows
            .into_iter()
            .map(|row| {
                let variant_id = ProductVariantId::new(row.id.to_string())?;
                let product_id = ProductVariantProductId::new(row.product_id as u32)?;
                
                // 色オブジェクトを作成
                let color_name = ColorName::new(row.color_name)?;
                let color = Color::new(
                    row.color_id as u32, 
                    color_name, 
                    row.color_hex, 
                    None, 
                    None
                )?;
                
                ProductVariant::new(
                    variant_id,
                    product_id,
                    row.name,
                    Money::from_yen(row.base_price as u32),
                    row.sale_price.map(|p| Money::from_yen(p as u32)),
                    Some(color),
                    row.size,
                    row.image_url,
                    row.is_available,
                    row.stock_quantity as u32,
                    row.reserved_quantity as u32,
                    row.low_stock_threshold.map(|t| t as u32),
                    row.cost_price.map(|p| Money::from_yen(p as u32)),
                )
            })
            .collect();
        let variants = variants.map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // 完全なProductオブジェクトを構築
        let product = self.build_complete_product(product_row, color, images, tags, variants).await?;
        
        Ok(Some(product))
    }

    async fn save(&self, product: Product) -> Result<(), RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        let pool = db.get_pool();
        
        let entity = ProductEntity::from_domain(&product);
        
        // 既存のプロダクトを検索
        let existing = sqlx::query_as::<_, ProductEntity>(
            "SELECT * FROM products WHERE id = ?"
        )
        .bind(entity.id)
        .fetch_optional(pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
        
        match existing {
            // 更新
            Some(_) => {
                sqlx::query(
                    r#"
                    UPDATE products 
                    SET name = ?, description = ?, material = ?, dimensions = ?,
                        color_id = ?, base_price = ?, sale_price = ?, category_id = ?, 
                        stock_quantity = ?, reserved_quantity = ?, low_stock_threshold = ?,
                        has_variants = ?, is_active = ?, is_best_seller = ?, is_quick_ship = ?,
                        updated_at = ?
                    WHERE id = ?
                    "#
                )
                .bind(&entity.name)
                .bind(&entity.description)
                .bind(&entity.material)
                .bind(&entity.dimensions)
                .bind(entity.color_id)
                .bind(entity.base_price)
                .bind(entity.sale_price)
                .bind(entity.category_id)
                .bind(entity.stock_quantity)
                .bind(entity.reserved_quantity)
                .bind(entity.low_stock_threshold)
                .bind(entity.has_variants)
                .bind(entity.is_active)
                .bind(entity.is_best_seller)
                .bind(entity.is_quick_ship)
                .bind(&entity.updated_at)
                .bind(entity.id)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            },
            // 新規作成
            None => {
                sqlx::query(
                    r#"
                    INSERT INTO products (
                        name, description, material, dimensions,
                        color_id, base_price, sale_price, category_id, 
                        stock_quantity, reserved_quantity, low_stock_threshold,
                        has_variants, is_active, is_best_seller, is_quick_ship,
                        created_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#
                )
                .bind(&entity.name)
                .bind(&entity.description)
                .bind(&entity.material)
                .bind(&entity.dimensions)
                .bind(entity.color_id)
                .bind(entity.base_price)
                .bind(entity.sale_price)
                .bind(entity.category_id)
                .bind(entity.stock_quantity)
                .bind(entity.reserved_quantity)
                .bind(entity.low_stock_threshold)
                .bind(entity.has_variants)
                .bind(entity.is_active)
                .bind(entity.is_best_seller)
                .bind(entity.is_quick_ship)
                .bind(&entity.created_at)
                .bind(&entity.updated_at)
                .execute(pool)
                .await
                .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;
            }
        }
        
        Ok(())
    }
}
