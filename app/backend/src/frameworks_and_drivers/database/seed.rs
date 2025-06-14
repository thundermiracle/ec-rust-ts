use anyhow::Result;
use sqlx::SqlitePool;

use crate::frameworks_and_drivers::database::db::get_db;

/// シードデータ投入のメイン関数
pub async fn run_seeds() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    
    println!("🌱 Starting seed data insertion...");
    
    // システムタグを挿入
    seed_system_tags(pool).await?;
    println!("🏷️  System tags seeded");
    
    // 色を挿入
    seed_colors(pool).await?;
    println!("🎨 Colors seeded");
    
    // カテゴリーを挿入
    seed_categories(pool).await?;
    println!("📂 Categories seeded");
    
    println!("✅ All seed data inserted successfully!");
    Ok(())
}

/// システムタグを挿入
async fn seed_system_tags(pool: &SqlitePool) -> Result<()> {
    let tags = [
        ("on_sale", "On Sale", "#FF6B6B", 1),
        ("best_seller", "Best Seller", "#4ECDC4", 2),
        ("quick_ship", "Quick Ship", "#45B7D1", 3),
        ("new_arrival", "New Arrival", "#96CEB4", 4),
        ("sold_out", "Sold Out", "#FFEAA7", 5),
    ];
    
    for (slug, name, color, priority) in tags {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO tags (slug, name, color_code, priority)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(slug)
        .bind(name)
        .bind(color)
        .bind(priority)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// 色を挿入（mockDataから）
async fn seed_colors(pool: &SqlitePool) -> Result<()> {
    let colors = [
        ("Walnut", "#8B4513", 1),
        ("White Oak", "#F5F5DC", 2),
        ("Black Oak", "#2F2F2F", 3),
        ("Whitewash Oak", "#F8F8FF", 4),
        ("Black", "#000000", 5),
        ("White", "#FFFFFF", 6),
        ("Charcoal", "#36454F", 7),
        ("Mist", "#C4C4C4", 8),
        ("Smoke", "#738276", 9),
        ("Sand", "#C2B280", 10),
        ("Gray", "#808080", 11),
        ("Brass", "#B5A642", 12),
        ("Beige", "#F5F5DC", 13),
    ];
    
    for (name, hex_code, display_order) in colors {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO colors (name, hex_code, display_order)
            VALUES (?, ?, ?)
            "#
        )
        .bind(name)
        .bind(hex_code)
        .bind(display_order)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// カテゴリーを挿入（mockDataから）
async fn seed_categories(pool: &SqlitePool) -> Result<()> {
    let categories = [
        ("desks", "Desks", None::<i64>, 1),
        ("tables", "Tables", None::<i64>, 2),
        ("seating", "Seating", None::<i64>, 3),
        ("bench-consoles", "Bench and Consoles", None::<i64>, 4),
        ("accessories", "Accessories", None::<i64>, 5),
        ("monitor-risers", "Monitor Risers", None::<i64>, 6),
        ("wall-shelves", "Wall Shelves", None::<i64>, 7),
        ("side-tables", "Side Tables", None::<i64>, 8),
        ("lighting", "Lighting", None::<i64>, 9),
        ("bike-racks", "Bike Racks", None::<i64>, 10),
        ("audio", "Audio", None::<i64>, 11),
    ];
    
    for (slug, name, _parent_id, display_order) in categories {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO categories (slug, name, display_order)
            VALUES (?, ?, ?)
            "#
        )
        .bind(slug)
        .bind(name)
        .bind(display_order)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// mockDataから商品をサンプル挿入
pub async fn seed_sample_products() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    
    println!("🛍️  Seeding sample products from mockData...");
    
    // mockDataから選択した商品（USD→JPY変換: x150）
    let products = [
        // Desk - Walnut (on sale + best seller)
        ("desk-walnut-1", "Desk - Walnut", "Minimalist walnut desk with clean lines and modern design", 
         "Walnut Wood", "48\" x 24\" x 30\"", 343500, Some(268500), "desks", 15, true, true, false),
        
        // Lift - Walnut (quick ship)
        ("lift-walnut-1", "Lift - Walnut", "Monitor stand with storage space underneath",
         "Walnut Wood", "24\" x 10\" x 6\"", 58500, None, "monitor-risers", 25, false, false, true),
        
        // Form Armchair Swivel (on sale)
        ("form-armchair-swivel-1", "Form Armchair Swivel - Upholstered", "Comfortable swivel armchair with premium upholstery",
         "Fabric and Metal", "28\" x 28\" x 32\"", 257250, Some(201750), "seating", 8, false, false, false),
        
        // Coffee Table - Walnut (best seller)
        ("coffee-table-walnut", "Coffee Table - Walnut", "Modern coffee table with clean walnut finish",
         "Walnut Wood", "40\" x 20\" x 16\"", 180000, None, "tables", 12, false, true, false),
        
        // Bookshelf - White Oak (on sale)
        ("bookshelf-white-oak", "Bookshelf - White Oak", "Five-tier bookshelf with white oak finish",
         "White Oak Wood", "32\" x 12\" x 72\"", 133500, Some(112500), "wall-shelves", 6, false, false, false),
        
        // Pendant Light - Brass (quick ship)
        ("pendant-light-brass", "Pendant Light - Brass", "Modern pendant light with brass finish",
         "Brass and Glass", "8\" x 8\" x 12\"", 36000, None, "lighting", 20, false, false, true),
        
        // Table - Black Oak (sold out)
        ("table-black-oak-1", "Table - Black Oak", "Dining table with black oak finish",
         "Black Oak Wood", "72\" x 36\" x 30\"", 420000, None, "tables", 0, false, false, false),
        
        // Monument - Charcoal (quick ship)
        ("monument-charcoal-1", "Monument - Charcoal", "Phone stand with minimalist design",
         "Metal", "4\" x 4\" x 6\"", 13500, None, "accessories", 50, false, false, true),
    ];
    
    for (id, name, description, material, dimensions, base_price, sale_price, category_slug, quantity, is_best_seller, _is_on_sale, is_quick_ship) in products {
        // カテゴリーIDを取得
        let category_id: i64 = sqlx::query_scalar(
            "SELECT id FROM categories WHERE slug = ?"
        )
        .bind(category_slug)
        .fetch_one(pool)
        .await?;
        
        // 商品を挿入
        let product_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT OR IGNORE INTO products (
                name, description, material, dimensions, base_price, sale_price,
                category_id, quantity, is_active, is_best_seller, is_quick_ship
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, TRUE, ?, ?)
            RETURNING id
            "#
        )
        .bind(name)
        .bind(description)
        .bind(material)
        .bind(dimensions)
        .bind(base_price)
        .bind(sale_price)
        .bind(category_id)
        .bind(quantity)
        .bind(is_best_seller)
        .bind(is_quick_ship)
        .fetch_optional(pool)
        .await?;
        
        if let Some(product_id) = product_id {
            // 商品画像を挿入（mockDataのimagesから）
            let images = match id {
                "desk-walnut-1" => vec![
                    ("https://picsum.photos/id/100/800/800", "Desk - Walnut - Main View", true, 1),
                    ("https://picsum.photos/id/101/800/800", "Desk - Walnut - Side View", false, 2),
                ],
                "lift-walnut-1" => vec![
                    ("https://picsum.photos/id/102/800/800", "Lift - Walnut - Main View", true, 1),
                    ("https://picsum.photos/id/103/800/800", "Lift - Walnut - Detail View", false, 2),
                ],
                "form-armchair-swivel-1" => vec![
                    ("https://picsum.photos/id/312/800/800", "Form Armchair - Main View", true, 1),
                    ("https://picsum.photos/id/315/800/800", "Form Armchair - Side View", false, 2),
                ],
                "coffee-table-walnut" => vec![
                    ("https://picsum.photos/id/888/800/800", "Coffee Table - Main View", true, 1),
                    ("https://picsum.photos/id/890/800/800", "Coffee Table - Detail View", false, 2),
                ],
                "bookshelf-white-oak" => vec![
                    ("https://picsum.photos/id/524/800/800", "Bookshelf - Main View", true, 1),
                    ("https://picsum.photos/id/526/800/800", "Bookshelf - Detail View", false, 2),
                ],
                "pendant-light-brass" => vec![
                    ("https://picsum.photos/id/169/800/800", "Pendant Light - Main View", true, 1),
                ],
                "table-black-oak-1" => vec![
                    ("https://picsum.photos/id/115/800/800", "Table - Main View", true, 1),
                    ("https://picsum.photos/id/116/800/800", "Table - Detail View", false, 2),
                ],
                "monument-charcoal-1" => vec![
                    ("https://picsum.photos/id/431/800/800", "Monument - Main View", true, 1),
                ],
                _ => vec![],
            };
            
            for (image_url, alt_text, is_main, sort_order) in images {
                sqlx::query(
                    r#"
                    INSERT OR IGNORE INTO product_images (
                        product_id, image_url, alt_text, is_main, sort_order
                    )
                    VALUES (?, ?, ?, ?, ?)
                    "#
                )
                .bind(product_id)
                .bind(image_url)
                .bind(alt_text)
                .bind(is_main)
                .bind(sort_order)
                .execute(pool)
                .await?;
            }
            
            // 商品タグを挿入
            let tags = match id {
                "desk-walnut-1" => vec!["on_sale", "best_seller"],
                "lift-walnut-1" => vec!["quick_ship"],
                "form-armchair-swivel-1" => vec!["on_sale"],
                "coffee-table-walnut" => vec!["best_seller"],
                "bookshelf-white-oak" => vec!["on_sale"],
                "pendant-light-brass" => vec!["quick_ship"],
                "table-black-oak-1" => vec!["sold_out"],
                "monument-charcoal-1" => vec!["quick_ship"],
                _ => vec![],
            };
            
            for tag_slug in tags {
                let tag_id: i64 = sqlx::query_scalar(
                    "SELECT id FROM tags WHERE slug = ?"
                )
                .bind(tag_slug)
                .fetch_one(pool)
                .await?;
                
                sqlx::query(
                    r#"
                    INSERT OR IGNORE INTO product_tags (product_id, tag_id)
                    VALUES (?, ?)
                    "#
                )
                .bind(product_id)
                .bind(tag_id)
                .execute(pool)
                .await?;
            }
            
            // 在庫データを挿入
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO inventory (
                    product_id, total_quantity, reserved_quantity, 
                    low_stock_threshold
                )
                VALUES (?, ?, 0, 5)
                "#
            )
            .bind(product_id)
            .bind(quantity)
            .execute(pool)
            .await?;
            
            // 商品色を挿入（mockDataのcolorsから）
            let colors = match id {
                "desk-walnut-1" => vec!["Walnut"],
                "lift-walnut-1" => vec!["Walnut"],
                "form-armchair-swivel-1" => vec!["Black", "Gray", "Beige"],
                "coffee-table-walnut" => vec!["Walnut"],
                "bookshelf-white-oak" => vec!["White Oak"],
                "pendant-light-brass" => vec!["Brass"],
                "table-black-oak-1" => vec!["Black Oak"],
                "monument-charcoal-1" => vec!["Charcoal"],
                _ => vec![],
            };
            
            for color_name in colors {
                let color_id: i64 = sqlx::query_scalar(
                    "SELECT id FROM colors WHERE name = ?"
                )
                .bind(color_name)
                .fetch_one(pool)
                .await?;
                
                sqlx::query(
                    r#"
                    INSERT OR IGNORE INTO product_colors (product_id, color_id)
                    VALUES (?, ?)
                    "#
                )
                .bind(product_id)
                .bind(color_id)
                .execute(pool)
                .await?;
            }
        }
    }
    
    println!("🛍️  Sample products seeded from mockData");
    Ok(())
} 