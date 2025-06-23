use async_trait::async_trait;
use sqlx::{Row, SqlitePool};

use crate::application::error::RepositoryError;
use crate::application::repositories::ProductRepository;
use crate::application::dto::{ProductListDTO, ProductSummaryDTO, ProductDTO, VariantDTO};
use crate::domain::models::ProductId;
use crate::infrastructure::database::db::get_db;

/// SQLite実装のProductRepository
/// Clean Architecture: Frameworks & Drivers層
/// CQRS Query側専用：ProductDTOを直接構築してパフォーマンス重視
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
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<ProductDTO>, RepositoryError> {
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

        // SKU情報（バリアント）を取得 - display_orderを使用したハイブリッドソート
        let sku_rows = sqlx::query(
            r#"
            SELECT 
                s.id,
                s.sku_code,
                s.name,
                s.dimensions,
                s.material,
                s.base_price,
                s.sale_price,
                s.stock_quantity,
                s.reserved_quantity,
                s.display_order,
                s.image_url,
                c.name as color_name,
                c.hex as color_hex
            FROM skus s
            JOIN colors c ON c.id = s.color_id
            WHERE s.product_id = ?
            ORDER BY 
                s.display_order ASC,
                CASE WHEN s.stock_quantity - s.reserved_quantity > 0 THEN 0 ELSE 1 END,
                COALESCE(s.sale_price, s.base_price) ASC,
                s.dimensions ASC,
                c.name ASC
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

        for sku_row in sku_rows {
            let sku_id: String = sku_row.try_get("id")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sku_code: String = sku_row.try_get("sku_code")
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
            let display_order: i64 = sku_row.try_get("display_order")
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
            let available_stock = (stock_quantity - reserved_quantity).max(0) as u32;

            // 価格計算（JPYは最小単位で保存されているため100で割る）
            let variant_price = base_price as u32;
            let variant_sale_price = sku_sale_price.map(|p| p  as u32);

            // バリアント作成 - VariantDTOの構造体フィールドに直接設定
            variants.push(VariantDTO::new(
                sku_id,
                sku_code,
                sku_name,
                color_name,
                sku_material.unwrap_or_default(),
                sku_dimensions.unwrap_or_default(),
                variant_price,
                variant_sale_price,
                available_stock,
                display_order as u32,
                sku_image_url,
            ));
        }

        // 画像URLリストを構築
        let images: Vec<String> = image_rows
            .iter()
            .map(|row| row.try_get("image_url"))
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

        // ProductDTOを構築 - 新しい構造に合わせて直接フィールド設定
        let product_dto = ProductDTO {
            id: product_id_str,
            name,
            images,
            category: category_name,
            description,
            is_best_seller,
            is_quick_ship,
            variants,
        };

        Ok(Some(product_dto))
    }

    async fn find_all(&self) -> Result<ProductListDTO, RepositoryError> {
        let pool = self.get_pool().await?;

        // 各商品の最初のSKUのデータを取得（ROW_NUMBERを使用してパフォーマンス最適化）
        let product_rows = sqlx::query(
            r#"
            WITH first_sku AS (
                SELECT 
                    s.product_id,
                    s.base_price,
                    s.sale_price,
                    s.stock_quantity,
                    s.reserved_quantity,
                    ROW_NUMBER() OVER (
                        PARTITION BY s.product_id 
                        ORDER BY 
                            s.display_order ASC,
                            CASE WHEN s.stock_quantity - s.reserved_quantity > 0 THEN 0 ELSE 1 END,
                            s.base_price ASC,
                            s.id ASC
                    ) as rn
                FROM skus s
            )
            SELECT 
                p.id,
                p.name,
                p.is_best_seller,
                p.is_quick_ship,
                c.name as category_name,
                fs.base_price,
                fs.sale_price,
                fs.stock_quantity,
                fs.reserved_quantity,
                pi.image_url as first_image
            FROM products p
            JOIN categories c ON c.id = p.category_id
            JOIN first_sku fs ON fs.product_id = p.id AND fs.rn = 1
            LEFT JOIN (
                SELECT product_id, image_url,
                       ROW_NUMBER() OVER (PARTITION BY product_id ORDER BY display_order) as rn
                FROM product_images
            ) pi ON pi.product_id = p.id AND pi.rn = 1
            ORDER BY p.name
            "#
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        if product_rows.is_empty() {
            let product_list = ProductListDTO {
                products: Vec::new(),
                total_count: 0,
                page: 1,
                per_page: 0,
                has_next_page: false,
                has_previous_page: false,
            };
            return Ok(product_list);
        }

        // 全商品のIDを収集
        let product_ids: Vec<String> = product_rows
            .iter()
            .map(|row| row.try_get("id"))
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

        // プレースホルダーを動的に生成
        let placeholders = product_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

        // 各商品の色情報を取得
        let color_query = format!(
            r#"
            SELECT DISTINCT
                s.product_id,
                colors.name as color_name
            FROM skus s
            JOIN colors ON colors.id = s.color_id
            WHERE s.product_id IN ({})
            ORDER BY s.product_id, colors.name
            "#,
            placeholders
        );

        let mut color_query_builder = sqlx::query(&color_query);
        for product_id in &product_ids {
            color_query_builder = color_query_builder.bind(product_id);
        }

        let color_rows = color_query_builder
            .fetch_all(&pool)
            .await
            .map_err(|e| RepositoryError::QueryExecution(e.to_string()))?;

        let mut product_summaries = Vec::new();

        for product_row in product_rows {
            let product_id: String = product_row.try_get("id")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let name: String = product_row.try_get("name")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let is_best_seller: bool = product_row.try_get("is_best_seller")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let is_quick_ship: bool = product_row.try_get("is_quick_ship")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let category_name: String = product_row.try_get("category_name")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let base_price: i64 = product_row.try_get("base_price")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let sale_price: Option<i64> = product_row.try_get("sale_price")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let stock_quantity: i64 = product_row.try_get("stock_quantity")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let reserved_quantity: i64 = product_row.try_get("reserved_quantity")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;
            let first_image: Option<String> = product_row.try_get("first_image")
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

            // この商品の色情報を取得
            let product_colors: Vec<String> = color_rows
                .iter()
                .filter(|row| {
                    row.try_get::<String, _>("product_id")
                        .map(|id| id == product_id)
                        .unwrap_or(false)
                })
                .map(|row| row.try_get("color_name"))
                .collect::<Result<Vec<String>, _>>()
                .map_err(|e| RepositoryError::DataConversionError(e.to_string()))?;

            // 価格計算（JPYは最小単位で保存されているため100で割る）
            let product_price = base_price as u32;
            let sale_price_converted = sale_price.map(|p| p as u32);
            let available_stock = (stock_quantity - reserved_quantity).max(0) as u32;

            let product_summary = ProductSummaryDTO::new(
                product_id,
                name,
                category_name,
                product_price,
                sale_price_converted,
                first_image,
                product_colors, // 色情報を追加
                is_best_seller,
                is_quick_ship,
                available_stock,
            );

            product_summaries.push(product_summary);
        }

        let total_count = product_summaries.len() as u32;

        // 簡単な実装：ページング無しでの全件返却
        // 実際のプロダクションではLIMIT/OFFSETでページングを実装する
        let product_list = ProductListDTO {
            products: product_summaries,
            total_count,
            page: 1,
            per_page: total_count,
            has_next_page: false,
            has_previous_page: false,
        };

        Ok(product_list)
    }
}
