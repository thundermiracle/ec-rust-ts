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
        ("on_sale", "On Sale", 1),
        ("best_seller", "Best Seller", 2),
        ("quick_ship", "Quick Ship", 3),
        ("new_arrival", "New Arrival", 4),
        ("sold_out", "Sold Out", 5),
    ];
    
    for (slug, name, priority) in tags {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO tags (slug, name, priority, is_system)
            VALUES (?, ?, ?, TRUE)
            "#
        )
        .bind(slug)
        .bind(name)
        .bind(priority)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// 色を挿入（中央集権的な色マスターテーブル）
async fn seed_colors(pool: &SqlitePool) -> Result<()> {
    let colors = [
        ("Walnut", "#8B4513"),
        ("White Oak", "#F5F5DC"),
        ("Natural Bamboo", "#D2B48C"),
        ("Black Oak", "#2F2F2F"),
        ("Whitewash Oak", "#F8F8FF"),
        ("Black", "#000000"),
        ("White", "#FFFFFF"),
        ("Charcoal", "#36454F"),
        ("Mist", "#C4C4C4"),
        ("Smoke", "#738276"),
        ("Sand", "#C2B280"),
        ("Gray", "#808080"),
        ("Brass", "#B5A642"),
        ("Beige", "#F5F5DC"),
    ];
    
    for (name, hex) in colors {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO colors (name, hex)
            VALUES (?, ?)
            "#
        )
        .bind(name)
        .bind(hex)
        .execute(pool)
        .await?;
    }
    
    Ok(())
}

/// カテゴリーを挿入
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
    
    for (slug, name, parent_id, display_order) in categories {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO categories (slug, name, parent_id, display_order)
            VALUES (?, ?, ?, ?)
            "#
        )
        .bind(slug)
        .bind(name)
        .bind(parent_id)
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
    
    // シンプル商品（バリアントなし）
    let simple_products = [
        // Desk Organizer - バリアントなしの商品
        (
            "Desk Organizer", 
            "DO-BAMBOO-001", 
            "Minimalist desk organizer", 
            "Bamboo", 
            "W20×D15×H8cm",
            "Natural Bamboo", // color_name
            4500, // base_price
            None::<i64>, // sale_price
            "accessories", // category_slug
            25, // stock_quantity
            false, // is_best_seller
            false  // is_quick_ship
        ),
        // Pendant Light - バリアントなしの商品
        (
            "Pendant Light - Brass", 
            "PL-BRASS-001", 
            "Modern pendant light with brass finish", 
            "Brass and Glass", 
            "8\" x 8\" x 12\"",
            "Brass", // color_name
            36000, // base_price
            None::<i64>, // sale_price
            "lighting", // category_slug
            20, // stock_quantity
            false, // is_best_seller
            true  // is_quick_ship
        ),
    ];
    
    // バリアント商品
    let variant_products = [
        // Coffee Table - バリアントあり
        (
            "Coffee Table", 
            "CT-ROUND-001", 
            "Round coffee table", 
            "Oak wood", 
            "tables", // category_slug
            vec![
                // バリアント: (sku, name, color_name, size, base_price, sale_price, stock_quantity)
                ("CT-WALNUT-SMALL", "Small – Walnut", "Walnut", "Small", 160000, None::<i64>, 8),
                ("CT-WALNUT-LARGE", "Large – Walnut", "Walnut", "Large", 180000, None::<i64>, 5),
                ("CT-OAK-SMALL", "Small – White Oak", "White Oak", "Small", 160000, None::<i64>, 12),
                ("CT-OAK-LARGE", "Large – White Oak", "White Oak", "Large", 180000, None::<i64>, 7),
            ]
        ),
        // Form Armchair - バリアントあり
        (
            "Form Armchair Swivel", 
            "FA-SWIVEL-001", 
            "Comfortable swivel armchair with premium upholstery", 
            "Fabric and Metal", 
            "seating", // category_slug
            vec![
                // バリアント: (sku, name, color_name, size, base_price, sale_price, stock_quantity)
                ("FA-BLACK-STD", "Black Upholstery", "Black", "Standard", 201750, Some(180000), 4),
                ("FA-GRAY-STD", "Gray Upholstery", "Gray", "Standard", 201750, Some(180000), 4),
            ]
        ),
    ];
    
    // シンプル商品（バリアントなし）を挿入
    for (name, sku, description, material, dimensions, color_name, base_price, sale_price, category_slug, stock_quantity, is_best_seller, is_quick_ship) in simple_products {
        // カテゴリーIDを取得
        let category_id: i64 = sqlx::query_scalar(
            "SELECT id FROM categories WHERE slug = ?"
        )
        .bind(category_slug)
        .fetch_one(pool)
        .await?;
        
        // 色IDを取得
        let color_id: i64 = sqlx::query_scalar(
            "SELECT id FROM colors WHERE name = ?"
        )
        .bind(color_name)
        .fetch_one(pool)
        .await?;
        
        // 商品を挿入
        let product_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO products (
                name, sku, description, material, dimensions,
                color_id, category_id, base_price, sale_price,
                stock_quantity, reserved_quantity, has_variants,
                is_active, is_best_seller, is_quick_ship
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, FALSE, TRUE, ?, ?)
            RETURNING id
            "#
        )
        .bind(name)
        .bind(sku)
        .bind(description)
        .bind(material)
        .bind(dimensions)
        .bind(color_id)
        .bind(category_id)
        .bind(base_price)
        .bind(sale_price)
        .bind(stock_quantity)
        .bind(is_best_seller)
        .bind(is_quick_ship)
        .fetch_one(pool)
        .await?;
        
        // サンプル画像を挿入
        sqlx::query(
            r#"
            INSERT INTO product_images (product_id, image_url, alt_text, display_order)
            VALUES (?, ?, ?, 0)
            "#
        )
        .bind(product_id)
        .bind(format!("https://picsum.photos/id/{}/800/800", 100 + product_id))
        .bind(format!("Image of {}", name))
        .execute(pool)
        .await?;
    }
    
    // バリアント商品を挿入
    for (name, sku, description, material, category_slug, variants) in variant_products {
        // カテゴリーIDを取得
        let category_id: i64 = sqlx::query_scalar(
            "SELECT id FROM categories WHERE slug = ?"
        )
        .bind(category_slug)
        .fetch_one(pool)
        .await?;
        
        // 親商品を挿入（has_variants = TRUE）
        let product_id = sqlx::query_scalar::<_, i64>(
            r#"
            INSERT INTO products (
                name, sku, description, material, category_id,
                has_variants, is_active, is_best_seller, is_quick_ship
            )
            VALUES (?, ?, ?, ?, ?, TRUE, TRUE, FALSE, FALSE)
            RETURNING id
            "#
        )
        .bind(name)
        .bind(sku)
        .bind(description)
        .bind(material)
        .bind(category_id)
        .fetch_one(pool)
        .await?;
        
        // サンプル画像を挿入（親商品用）
        sqlx::query(
            r#"
            INSERT INTO product_images (product_id, image_url, alt_text, is_main)
            VALUES (?, ?, ?, TRUE)
            "#
        )
        .bind(product_id)
        .bind(format!("https://picsum.photos/id/{}/800/800", 200 + product_id))
        .bind(format!("Image of {}", name))
        .execute(pool)
        .await?;
        
        // バリアントを挿入
        for (variant_idx, (variant_sku, variant_name, color_name, dimensions, base_price, sale_price, stock_quantity)) in variants.iter().enumerate() {
            // 色IDを取得
            let color_id: i64 = sqlx::query_scalar(
                "SELECT id FROM colors WHERE name = ?"
            )
            .bind(color_name)
            .fetch_one(pool)
            .await?;
            
            // バリアントを挿入
            let variant_id = sqlx::query_scalar::<_, i64>(
                r#"
                INSERT INTO product_variants (
                    product_id, sku, name, color_id, dimensions,
                    base_price, sale_price, stock_quantity, reserved_quantity,
                    is_available, image_url
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, 0, TRUE, ?)
                RETURNING id
                "#
            )
            .bind(product_id)
            .bind(variant_sku)
            .bind(variant_name)
            .bind(color_id)
            .bind(dimensions)
            .bind(base_price)
            .bind(sale_price)
            .bind(stock_quantity)
            .bind(format!("https://picsum.photos/id/{}/800/800", 300 + product_id + variant_idx as i64))
            .fetch_one(pool)
            .await?;
            
            println!("  ↳ Variant created: {} (ID: {})", variant_name, variant_id);
        }
    }
    
    println!("✅ Sample products seeded successfully!");
    Ok(())
} 