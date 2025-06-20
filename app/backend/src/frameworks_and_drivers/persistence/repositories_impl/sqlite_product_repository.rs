use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::error::RepositoryError;
use crate::application::repositories::ProductRepository;
use crate::application::viewmodels::{ProductViewModel, VariantViewModel};
use crate::domain::models::ProductId;
use crate::frameworks_and_drivers::database::db::get_db;

/// SQLite実装のProductRepository
/// Clean Architecture: Frameworks & Drivers層
/// CQRS Query側専用：ProductViewModelを直接構築してパフォーマンス重視
pub struct SqliteProductRepository {
    pool: SqlitePool,
}

impl SqliteProductRepository {
    pub fn new() -> Self {
        // データベースプールは実際のクエリ実行時に取得する
        // 初期化時に非同期で取得することはできないため、プレースホルダーとして空のプールを使用
        let pool = SqlitePool::connect_lazy("sqlite::memory:").unwrap();
        Self { pool }
    }

    /// データベースプールを取得（実際のクエリ実行時に使用）
    async fn get_pool(&self) -> Result<SqlitePool, RepositoryError> {
        get_db().await
            .map(|db| db.get_pool().clone())
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductViewModel>, RepositoryError> {
        let pool = self.get_pool().await?;
        let product_id_str = id.value().to_string();

        // 商品基本情報とカテゴリー名を取得
        let product_row = sqlx::query(
            r#"
            SELECT 
                p.id,
                p.name,
                p.description,
                p.is_best_seller,
                p.is_quick_ship,
                c.name as category_name
            FROM products p
            JOIN categories c ON c.id = p.category_id
            WHERE p.id = ?
            "#
        )
        .bind(&product_id_str)
        .fetch_optional(&pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let product_row = match product_row {
            Some(row) => row,
            None => return Ok(None),
        };

        // SKU情報（バリアント）を取得
        let sku_rows = sqlx::query(
            r#"
            SELECT 
                s.id,
                s.name,
                s.dimensions,
                s.material,
                s.base_price,
                s.sale_price,
                s.stock_quantity,
                s.reserved_quantity,
                s.image_url,
                c.name as color_name,
                c.hex as color_hex
            FROM skus s
            JOIN colors c ON c.id = s.color_id
            WHERE s.product_id = ?
            ORDER BY s.name
            "#
        )
        .bind(&product_id_str)
        .fetch_all(&pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // 商品画像を取得
        let image_rows = sqlx::query(
            r#"
            SELECT image_url
            FROM product_images
            WHERE product_id = ?
            ORDER BY display_order
            "#
        )
        .bind(&product_id_str)
        .fetch_all(&pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        // データ変換処理
        let name: String = product_row.try_get("name")
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
        let description: String = product_row.try_get("description")
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
        let is_best_seller: bool = product_row.try_get("is_best_seller")
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
        let is_quick_ship: bool = product_row.try_get("is_quick_ship")
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
        let category_name: String = product_row.try_get("category_name")
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

        // バリアント情報を構築
        let mut variants = Vec::new();
        let mut min_price: Option<u32> = None;
        let mut has_sale_price = false;
        let mut sale_price: Option<u32> = None;
        let mut colors = Vec::new();
        let mut materials = Vec::new();
        let mut dimensions = Vec::new();
        let mut all_sold_out = true;

        for sku_row in sku_rows {
            let sku_id: String = sku_row.try_get("id")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_name: String = sku_row.try_get("name")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let base_price: i64 = sku_row.try_get("base_price")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_sale_price: Option<i64> = sku_row.try_get("sale_price")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let stock_quantity: i64 = sku_row.try_get("stock_quantity")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let reserved_quantity: i64 = sku_row.try_get("reserved_quantity")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_image_url: Option<String> = sku_row.try_get("image_url")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let color_name: String = sku_row.try_get("color_name")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_dimensions: Option<String> = sku_row.try_get("dimensions")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_material: Option<String> = sku_row.try_get("material")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

            // 在庫確認
            let available_stock = stock_quantity - reserved_quantity;
            let is_available = available_stock > 0;
            if is_available {
                all_sold_out = false;
            }

            // 価格計算（JPYは最小単位で保存されているため100で割る）
            let variant_price = (base_price / 100) as u32;
            let variant_sale_price = sku_sale_price.map(|p| (p / 100) as u32);

            // 最小価格の更新
            let current_price = variant_sale_price.unwrap_or(variant_price);
            min_price = Some(min_price.map_or(current_price, |existing| existing.min(current_price)));

            // セール価格があるかチェック
            if variant_sale_price.is_some() {
                has_sale_price = true;
                if sale_price.is_none() {
                    sale_price = variant_sale_price;
                }
            }

            // バリアント作成
            variants.push(VariantViewModel::new(
                sku_id,
                sku_name,
                variant_price,
                color_name.clone(),
                sku_image_url,
                is_available,
            ));

            // 色、素材、寸法の収集
            if !colors.contains(&color_name) {
                colors.push(color_name);
            }
            if let Some(material) = sku_material.clone() {
                if !materials.contains(&material) {
                    materials.push(material);
                }
            }
            if let Some(dim) = sku_dimensions.clone() {
                if !dimensions.contains(&dim) {
                    dimensions.push(dim);
                }
            }
        }

        // 画像URLリストを構築
        let images: Vec<String> = image_rows
            .iter()
            .map(|row| row.try_get("image_url"))
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

        // ProductViewModelを構築
        let product_view_model = ProductViewModel::new(
            product_id_str,
            name,
            min_price.unwrap_or(0),
            if has_sale_price { sale_price } else { None },
            images,
            category_name,
            description,
            materials.first().cloned(), // 最初の素材を代表として使用
            dimensions.first().cloned(), // 最初の寸法を代表として使用
            colors,
            has_sale_price,
            is_best_seller,
            is_quick_ship,
            all_sold_out,
            variants,
        );

        Ok(Some(product_view_model))
    }
}
