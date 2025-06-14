use serde::{Deserialize, Serialize};
use crate::application::queries::GetProductQuery;

/// Product Presenter - 商品情報の統合プレゼンター
/// 基本情報とリッチ情報を統合した単一のレスポンス構造
#[derive(Serialize, Deserialize)]
pub struct ProductPresenter {
    pub id: String,
    pub name: String,
    pub description: String,
    pub material: Option<String>,
    pub dimensions: Option<String>,
    
    // 価格情報
    pub price: u32,                    // 現在価格（セール価格 or 基本価格）
    pub base_price: u32,               // 基本価格
    pub sale_price: Option<u32>,       // セール価格
    pub discount_percentage: Option<u8>, // 割引率
    pub savings_amount: Option<u32>,   // 節約額
    
    // カテゴリー情報
    pub category: CategoryInfo,
    
    // 画像情報
    pub images: Vec<ProductImage>,
    pub main_image: Option<String>,
    
    // 色情報
    pub colors: Vec<String>,
    pub available_colors: Vec<ColorInfo>,
    
    // フラグ・タグ情報
    pub is_on_sale: bool,
    pub is_best_seller: bool,
    pub is_quick_ship: bool,
    pub is_sold_out: bool,
    pub is_new_arrival: bool,
    pub tags: Vec<TagInfo>,
    
    // 在庫情報
    pub quantity: u32,
    pub is_available: bool,
    pub stock_status: String,
}

/// カテゴリー情報
#[derive(Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: String,
    pub name: String,
    pub slug: String,
}

/// 商品画像情報
#[derive(Serialize, Deserialize)]
pub struct ProductImage {
    pub url: String,
    pub alt_text: Option<String>,
    pub is_main: bool,
    pub sort_order: u32,
}

/// 色情報
#[derive(Serialize, Deserialize)]
pub struct ColorInfo {
    pub name: String,
    pub hex_code: Option<String>,
    pub display_order: u32,
}

/// タグ情報
#[derive(Serialize, Deserialize)]
pub struct TagInfo {
    pub slug: String,
    pub name: String,
    pub color_code: Option<String>,
    pub priority: u8,
}

/// Application層のQueryからPresenterへの変換
impl From<GetProductQuery> for ProductPresenter {
    fn from(query: GetProductQuery) -> Self {
        ProductPresenter {
            id: query.id.to_string(),
            name: query.name.clone(),
            description: query.description.clone(),
            material: Some("Walnut Wood".to_string()),
            dimensions: Some("48\" x 24\" x 30\"".to_string()),
            
            // 価格情報（基本価格から10%割引のサンプル）
            price: query.price,
            base_price: (query.price as f64 * 1.11) as u32, // 逆算でbase_price
            sale_price: if query.id % 2 == 1 { Some(query.price) } else { None },
            discount_percentage: if query.id % 2 == 1 { Some(10) } else { None },
            savings_amount: if query.id % 2 == 1 { 
                Some((query.price as f64 * 0.11) as u32) 
            } else { 
                None 
            },
            
            // カテゴリー情報（サンプル）
            category: CategoryInfo {
                id: "desks".to_string(),
                name: "Desks".to_string(),
                slug: "desks".to_string(),
            },
            
            // 画像情報（サンプル）
            images: vec![
                ProductImage {
                    url: format!("https://picsum.photos/id/{}/800/800", query.id * 10),
                    alt_text: Some(format!("{} - Main View", query.name)),
                    is_main: true,
                    sort_order: 0,
                },
                ProductImage {
                    url: format!("https://picsum.photos/id/{}/800/800", query.id * 10 + 1),
                    alt_text: Some(format!("{} - Side View", query.name)),
                    is_main: false,
                    sort_order: 1,
                },
            ],
            main_image: Some(format!("https://picsum.photos/id/{}/800/800", query.id * 10)),
            
            // 色情報（サンプル）
            colors: vec!["Walnut".to_string()],
            available_colors: vec![
                ColorInfo {
                    name: "Walnut".to_string(),
                    hex_code: Some("#8B4513".to_string()),
                    display_order: 1,
                },
            ],
            
            // フラグ情報（サンプル）
            is_on_sale: query.id % 2 == 1,
            is_best_seller: query.id % 3 == 0,
            is_quick_ship: query.id % 4 == 0,
            is_sold_out: query.quantity == 0,
            is_new_arrival: query.id > 6,
            tags: vec![
                TagInfo {
                    slug: if query.id % 2 == 1 { "on_sale".to_string() } else { "best_seller".to_string() },
                    name: if query.id % 2 == 1 { "On Sale".to_string() } else { "Best Seller".to_string() },
                    color_code: Some("#e74c3c".to_string()),
                    priority: 1,
                },
            ],
            
            // 在庫情報
            quantity: query.quantity,
            is_available: query.quantity > 0,
            stock_status: if query.quantity > 10 {
                "In Stock".to_string()
            } else if query.quantity > 0 {
                "Low Stock".to_string()
            } else {
                "Out of Stock".to_string()
            },
        }
    }
}

impl ProductPresenter {
    /// 価格表示フォーマット
    pub fn format_price_display(&self) -> String {
        if let Some(sale_price) = self.sale_price {
            format!("¥{} (was ¥{})", sale_price, self.base_price)
        } else {
            format!("¥{}", self.base_price)
        }
    }
    
    /// 割引表示
    pub fn format_discount_display(&self) -> Option<String> {
        self.discount_percentage.map(|discount| format!("{}% OFF", discount))
    }

    /// 在庫状況表示
    pub fn format_availability_status(&self) -> String {
        if self.quantity > 0 {
            "在庫あり".to_string()
        } else {
            "在庫切れ".to_string()
        }
    }
} 