use crate::domain::models::{Product, ProductVariant};

/// Variant詳細情報
#[derive(Debug, Clone)]
pub struct VariantDetail {
    pub id: String,
    pub name: String,
    pub price: u32,
    pub color: String,
    pub image: Option<String>,
    pub is_available: bool,
}

/// リッチな商品詳細 - API応答に最適化された完全なDTO
/// Clean Architecture: Application層のQuery DTO
/// ビジネスロジックを含まず、データ転送のみを担当
#[derive(Debug, Clone)]
pub struct ProductDetail {
    // 基本情報
    pub id: u32,
    pub name: String,
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    
    // 価格情報
    pub base_price: u32,           // 基本価格（円）
    pub sale_price: Option<u32>,   // セール価格（円）
    pub current_price: u32,        // 現在の価格（円）
    pub discount_percentage: Option<u8>, // 割引率
    pub savings_amount: u32,       // 節約額（円）
    
    // カテゴリー情報
    pub category_id: u32,
    pub category_name: String,
    pub category_slug: String,
    
    // 画像情報
    pub images: Vec<String>,       // 画像URL配列
    pub main_image: Option<String>, // メイン画像URL
    
    // 色・タグ情報
    pub colors: Vec<String>,       // 利用可能な色配列
    pub tags: Vec<String>,         // タグ配列
    
    // 商品フラグ
    pub is_on_sale: bool,         // セール中
    pub is_best_seller: bool,     // ベストセラー
    pub is_quick_ship: bool,      // 迅速配送
    pub is_sold_out: bool,        // 売り切れ
    pub is_active: bool,          // アクティブ
    
    // 在庫情報
    pub quantity: u32,            // 在庫数
    pub is_available: bool,       // 購入可能
    
    // バリアント情報
    pub variants: Vec<VariantDetail>, // 商品バリアント配列
    
    // タイムスタンプ
    pub created_at: String,
    pub updated_at: String,
}

impl ProductDetail {
    /// 純粋なデータコンストラクタ（ビジネスロジックなし）
    /// 全ての計算済みフィールドは外部から提供される
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: u32,
        name: String,
        description: String,
        material: Option<String>,
        dimensions: Option<String>,
        base_price: u32,
        sale_price: Option<u32>,
        current_price: u32,
        discount_percentage: Option<u8>,
        savings_amount: u32,
        category_id: u32,
        category_name: String,
        category_slug: String,
        images: Vec<String>,
        main_image: Option<String>,
        colors: Vec<String>,
        tags: Vec<String>,
        is_on_sale: bool,
        is_best_seller: bool,
        is_quick_ship: bool,
        is_sold_out: bool,
        is_active: bool,
        quantity: u32,
        is_available: bool,
        variants: Vec<VariantDetail>,
        created_at: String,
        updated_at: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            material,
            dimensions,
            base_price,
            sale_price,
            current_price,
            discount_percentage,
            savings_amount,
            category_id,
            category_name,
            category_slug,
            images,
            main_image,
            colors,
            tags,
            is_on_sale,
            is_best_seller,
            is_quick_ship,
            is_sold_out,
            is_active,
            quantity,
            is_available,
            variants,
            created_at,
            updated_at,
        }
    }
}

impl From<ProductVariant> for VariantDetail {
    fn from(variant: ProductVariant) -> VariantDetail {
        VariantDetail {
            id: variant.id.value().to_string(),
            name: variant.name.clone(),
            price: variant.current_price().yen(),
            color: variant.color.clone(),
            image: variant.image_url.clone(),
            is_available: variant.is_available,
        }
    }
}

impl From<Product> for ProductDetail {
    fn from(product: Product) -> ProductDetail {
        // 必要な値を先に計算（borrowing conflicts回避のため）
        let current_price = product.current_price().yen();
        let discount_percentage = product.discount_percentage();
        let savings_amount = product.savings_amount().yen();
        let image_urls = product.image_urls();
        let main_image = product.main_image().map(|img| img.url().to_string());
        let color_names = product.color_names();
        let tag_names = product.tag_names();
        let is_sold_out = product.is_sold_out();
        let is_available_for_purchase = product.is_available_for_purchase();
        
        // ProductVariantをVariantDetailに変換
        let variants: Vec<VariantDetail> = product.variants
            .into_iter()
            .map(VariantDetail::from)
            .collect();
            
        ProductDetail {
            id: product.id,
            name: product.name.clone(),
            description: product.description.clone(),
            material: product.material.clone(),
            dimensions: product.dimensions.clone(),
            base_price: product.base_price.yen(),
            sale_price: product.sale_price.map(|price| price.yen()),
            current_price,
            discount_percentage,
            savings_amount,
            category_id: 1, // TODO: Use proper category ID conversion
            category_name: product.category.name.clone(),
            category_slug: product.category.slug.clone(),
            images: image_urls,
            main_image,
            colors: color_names,
            tags: tag_names,
            is_on_sale: product.is_on_sale,
            is_best_seller: product.is_best_seller,
            is_quick_ship: product.is_quick_ship,
            is_sold_out,
            is_active: product.is_available,
            quantity: product.quantity,
            is_available: is_available_for_purchase,
            variants,
            created_at: "2024-01-01T00:00:00Z".to_string(), // TODO: Add timestamps to Product
            updated_at: "2024-01-01T00:00:00Z".to_string(), // TODO: Add timestamps to Product
        }
    }
}

 