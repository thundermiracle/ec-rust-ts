use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

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
        ("desks", "Desks", None::<String>, 1),
        ("tables", "Tables", None::<String>, 2),
        ("seating", "Seating", None::<String>, 3),
        ("bench-consoles", "Bench and Consoles", None::<String>, 4),
        ("accessories", "Accessories", None::<String>, 5),
        ("monitor-risers", "Monitor Risers", None::<String>, 6),
        ("wall-shelves", "Wall Shelves", None::<String>, 7),
        ("side-tables", "Side Tables", None::<String>, 8),
        ("lighting", "Lighting", None::<String>, 9),
        ("bike-racks", "Bike Racks", None::<String>, 10),
        ("audio", "Audio", None::<String>, 11),
    ];
    
    for (slug, name, parent_id, display_order) in categories {
        let category_id = Uuid::new_v4().to_string();
        
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO categories (id, slug, name, parent_id, display_order)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
        .bind(&category_id)
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
    
    // シンプル商品（単一SKU）
    let simple_products = [
        // Desk Organizer
        (
            "Desk Organizer", 
            "Minimalist desk organizer", 
            "accessories", // category_slug
            false, // is_best_seller
            false, // is_quick_ship
            vec![
                // SKU: (sku_code, name, color_name, material, dimensions, base_price, sale_price, stock_quantity)
                ("DO-BAMBOO-001", "Natural Bamboo", "Natural Bamboo", "Bamboo", "W20×D15×H8cm", 4500, None::<i64>, 25)
            ]
        ),
        // Pendant Light
        (
            "Pendant Light", 
            "Modern pendant light with brass finish", 
            "lighting", // category_slug
            false, // is_best_seller
            true, // is_quick_ship
            vec![
                // SKU: (sku_code, name, color_name, material, dimensions, base_price, sale_price, stock_quantity)
                ("PL-BRASS-001", "Brass Finish", "Brass", "Brass and Glass", "8\" x 8\" x 12\"", 36000, None::<i64>, 20)
            ]
        ),
    ];
    
    // 複数SKU商品
    let multi_sku_products = [
        // Coffee Table - 複数SKU
        (
            "Coffee Table", 
            "Round coffee table", 
            "tables", // category_slug
            true, // is_best_seller
            false, // is_quick_ship
            vec![
                // SKU: (sku_code, name, color_name, material, dimensions, base_price, sale_price, stock_quantity)
                ("CT-WALNUT-SMALL", "Small – Walnut", "Walnut", "Solid Walnut", "Diameter: 80cm, Height: 45cm", 160000, None::<i64>, 8),
                ("CT-WALNUT-LARGE", "Large – Walnut", "Walnut", "Solid Walnut", "Diameter: 100cm, Height: 45cm", 180000, None::<i64>, 5),
                ("CT-OAK-SMALL", "Small – White Oak", "White Oak", "Solid Oak", "Diameter: 80cm, Height: 45cm", 160000, None::<i64>, 12),
                ("CT-OAK-LARGE", "Large – White Oak", "White Oak", "Solid Oak", "Diameter: 100cm, Height: 45cm", 180000, None::<i64>, 7)
            ]
        ),
        // Form Armchair - 複数SKU
        (
            "Form Armchair Swivel", 
            "Comfortable swivel armchair with premium upholstery", 
            "seating", // category_slug
            false, // is_best_seller
            false, // is_quick_ship
            vec![
                // SKU: (sku_code, name, color_name, material, dimensions, base_price, sale_price, stock_quantity)
                ("FA-BLACK-STD", "Black Upholstery", "Black", "Fabric and Metal", "Standard", 201750, Some(180000), 4),
                ("FA-GRAY-STD", "Gray Upholstery", "Gray", "Fabric and Metal", "Standard", 201750, Some(180000), 4)
            ]
        ),
    ];
    
    // すべての商品を処理（シンプルと複数SKU）
    for products in [simple_products, multi_sku_products].iter() {
        for (name, description, category_slug, is_best_seller, is_quick_ship, skus) in products.iter() {
            // カテゴリーIDを取得
            let category_id: String = sqlx::query_scalar(
                "SELECT id FROM categories WHERE slug = ?"
            )
            .bind(category_slug)
            .fetch_one(pool)
            .await?;
            
            // 商品IDを生成
            let product_id = Uuid::new_v4().to_string();
            
            // 商品を挿入
            sqlx::query(
                r#"
                INSERT INTO products (
                    id, name, description, category_id,
                    is_best_seller, is_quick_ship
                )
                VALUES (?, ?, ?, ?, ?, ?)
                "#
            )
            .bind(&product_id)
            .bind(name)
            .bind(description)
            .bind(&category_id)
            .bind(is_best_seller)
            .bind(is_quick_ship)
            .execute(pool)
            .await?;
            
            // サンプル画像を挿入
            sqlx::query(
                r#"
                INSERT INTO product_images (
                    product_id, image_url, alt_text, display_order
                )
                VALUES (?, ?, ?, 0)
                "#
            )
            .bind(&product_id)
            .bind(format!("https://picsum.photos/id/{}/800/800", 100 + skus.len() as i64))
            .bind(format!("Image of {}", name))
            .execute(pool)
            .await?;
            
            println!("  ✓ Product created: {} (ID: {})", name, product_id);
            
            // 各SKUを挿入
            for (sku_code, sku_name, color_name, material, dimensions, base_price, sale_price, stock_quantity) in skus {
                // 色IDを取得
                let color_id: i64 = sqlx::query_scalar(
                    "SELECT id FROM colors WHERE name = ?"
                )
                .bind(color_name)
                .fetch_one(pool)
                .await?;
                
                // SKUを挿入
                let sku_id = Uuid::new_v4().to_string();
                
                sqlx::query(
                    r#"
                    INSERT INTO skus (
                        id, product_id, sku_code, name,
                        color_id, material, dimensions,
                        base_price, sale_price, 
                        stock_quantity, reserved_quantity, low_stock_threshold,
                        image_url
                    )
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 5, ?)
                    "#
                )
                .bind(&sku_id)
                .bind(&product_id)
                .bind(sku_code)
                .bind(sku_name)
                .bind(color_id)
                .bind(material)
                .bind(dimensions)
                .bind(base_price)
                .bind(sale_price)
                .bind(stock_quantity)
                .bind(format!("https://picsum.photos/id/{}/800/800", 200 + *base_price % 100))
                .execute(pool)
                .await?;
                
                println!("    ↳ SKU created: {} (ID: {})", sku_name, sku_id);
            }
        }
    }
    
    println!("✅ Sample products seeded successfully!");
    Ok(())
} 