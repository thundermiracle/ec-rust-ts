use sqlx::{SqlitePool, query_as};
use async_trait::async_trait;
use uuid::Uuid;
use tokio::try_join;
use crate::frameworks_and_drivers::database::db::get_db;

use crate::application::repositories::ProductRepository;
use crate::application::error::RepositoryError;
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

/// 製品データの集約 - リポジトリから取得した関連データをまとめる構造体
#[derive(Debug)]
pub struct ProductAggregateData {
    pub product: ProductEntity,
    pub skus: Vec<SKUEntity>,
    pub images: Vec<ProductImageEntity>,
    pub tags: Vec<ProductTagEntity>,
    pub tag_details: Vec<TagEntity>,
    pub colors: Vec<ColorEntity>,
}

pub struct SqliteProductRepository;

impl SqliteProductRepository {
    pub fn new() -> Self {
        Self { }
    }

    async fn get_pool(&self) -> Result<SqlitePool, RepositoryError> {
        let db = get_db().await
            .map_err(|e| RepositoryError::DatabaseConnection(e.to_string()))?;
        Ok(db.get_pool().clone())
    }

    /// エンティティからドメインモデルへの変換
    async fn to_domain(&self, data: ProductAggregateData) -> Result<Product, DomainError> {
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
        
        // TODO: SKU、画像、タグなどの関連エンティティをドメインオブジェクトに追加する処理
        // この処理は現状の実装には含まれていませんが、必要に応じて追加します
        
        Ok(product)
    }

    /// 製品エンティティとその関連エンティティを取得
    async fn fetch_product_aggregate_by_id(&self, id: &ProductId) -> Result<Option<ProductAggregateData>, RepositoryError> {
        let pool = self.get_pool().await?;

        // 製品IDを文字列に変換
        let id_str = id.to_string();
        
        // 基本製品情報の取得
        let product = query_as::<_, ProductEntity>(
            "SELECT id, name, description, category_id, is_best_seller, is_quick_ship FROM products WHERE id = ?"
        )
        .bind(&id_str)
        .fetch_optional(&pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(format!("製品データの取得に失敗: {}", e)))?;

        let Some(product) = product else {
            return Ok(None);
        };

        // 並行して関連エンティティを取得
        let product_id = &product.id;
        
        // SQLクエリを定義
        let skus_query = query_as::<_, SKUEntity>(
            "SELECT id, product_id, color_id, size, price, stock FROM skus WHERE product_id = ?"
        )
        .bind(product_id);
        
        let images_query = query_as::<_, ProductImageEntity>(
            "SELECT id, product_id, url, display_order FROM product_images WHERE product_id = ? ORDER BY display_order"
        )
        .bind(product_id);
        
        let tags_query = query_as::<_, ProductTagEntity>(
            "SELECT id, product_id, tag_id FROM product_tags WHERE product_id = ?"
        )
        .bind(product_id);
        
        // 並行実行
        let (skus_result, images_result, product_tags_result) = 
            try_join!(
                skus_query.fetch_all(&pool),
                images_query.fetch_all(&pool),
                tags_query.fetch_all(&pool)
            )
            .map_err(|e| RepositoryError::DatabaseError(format!("関連データの取得に失敗: {}", e)))?;
        
        // タグIDとカラーIDのリストを取得
        let tag_ids: Vec<i64> = product_tags_result.iter().map(|pt| pt.tag_id).collect();
        let color_ids: Vec<i64> = skus_result.iter()
            .filter_map(|sku| sku.color_id)
            .collect();

        // タグ詳細とカラー詳細を取得
        let (tag_details, colors) = match (!tag_ids.is_empty(), !color_ids.is_empty()) {
            (false, false) => (Vec::new(), Vec::new()),
            _ => {
                // タグ詳細とカラー詳細を並行取得するための関数を定義
                async fn get_tag_details(pool: &SqlitePool, tag_ids: &[i64]) -> Result<Vec<TagEntity>, sqlx::Error> {
                    if tag_ids.is_empty() {
                        return Ok(Vec::new());
                    }
                    
                    // SQLのIN句用の（?,...）を構築
                    // タグの数だけプレースホルダを用意する安全な方法
                    let mut query = "SELECT id, name, category FROM tags WHERE id IN (".to_string();
                    query.push_str(&vec!["?"; tag_ids.len()].join(", "));
                    query.push(')');
                    
                    // クエリビルダーに値をバインド
                    let mut q = query_as::<_, TagEntity>(&query);
                    for id in tag_ids {
                        q = q.bind(id);
                    }
                    
                    q.fetch_all(pool).await
                }
                
                async fn get_color_details(pool: &SqlitePool, color_ids: &[i64]) -> Result<Vec<ColorEntity>, sqlx::Error> {
                    if color_ids.is_empty() {
                        return Ok(Vec::new());
                    }
                    
                    // SQLのIN句用の（?,...）を構築
                    let mut query = "SELECT id, name, code FROM colors WHERE id IN (".to_string();
                    query.push_str(&vec!["?"; color_ids.len()].join(", "));
                    query.push(')');
                    
                    // クエリビルダーに値をバインド
                    let mut q = query_as::<_, ColorEntity>(&query);
                    for id in color_ids {
                        q = q.bind(id);
                    }
                    
                    q.fetch_all(pool).await
                }
                
                // 並行して実行
                try_join!(
                    get_tag_details(&pool, &tag_ids),
                    get_color_details(&pool, &color_ids)
                )
                .map_err(|e| RepositoryError::DatabaseError(format!("詳細データの取得に失敗: {}", e)))?
            }
        };

        Ok(Some(ProductAggregateData {
            product,
            skus: skus_result,
            images: images_result,
            tags: product_tags_result,
            tag_details,
            colors,
        }))
    }
}

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_all(&self) -> Result<Vec<Product>, RepositoryError> {
        let pool = self.get_pool().await?;

        // 全ての製品IDを取得（コンパイル時のマクロを避け、通常のクエリを使用）
        #[derive(sqlx::FromRow)]
        struct IdRow {
            id: String,
        }
        
        let product_ids = query_as::<_, IdRow>(
            "SELECT id FROM products"
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        // 各製品の詳細を取得
        let mut products = Vec::with_capacity(product_ids.len());
        
        for row in product_ids {
            // 文字列からUUIDに変換してからProductIdに
            let uuid = Uuid::parse_str(&row.id)
                .map_err(|_| RepositoryError::DataConversionError("Invalid product ID format".to_string()))?;
                
            let product_id = ProductId::from_uuid(uuid);
            
            if let Some(aggregate) = self.fetch_product_aggregate_by_id(&product_id).await? {
                match self.to_domain(aggregate).await {
                    Ok(product) => products.push(product),
                    Err(e) => return Err(RepositoryError::DomainError(e.to_string())),
                }
            }
        }

        Ok(products)
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, RepositoryError> {
        // 製品とその関連データを取得（ProductIdを直接使用）
        let aggregate = match self.fetch_product_aggregate_by_id(id).await? {
            Some(data) => data,
            None => return Ok(None),
        };
        
        // ドメインモデルに変換
        self.to_domain(aggregate)
            .await
            .map_err(|e| RepositoryError::DomainError(e.to_string()))
            .map(Some)
    }
}
