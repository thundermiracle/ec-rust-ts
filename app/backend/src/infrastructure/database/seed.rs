use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::infrastructure::database::db::get_db;

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

    // サンプル商品を挿入
    seed_sample_products().await?;
    println!("🛍️  Sample products seeded");

    // サンプルクーポンを挿入
    seed_sample_coupons().await?;
    println!("🎫 Sample coupons seeded");

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
        let result = sqlx::query(
            r#"
            INSERT OR REPLACE INTO tags (slug, name, priority, is_system)
            VALUES (?, ?, ?, TRUE)
            "#,
        )
        .bind(slug)
        .bind(name)
        .bind(priority)
        .execute(pool)
        .await?;

        println!(
            "    ✓ Tag inserted: {} (affected rows: {})",
            slug,
            result.rows_affected()
        );
    }

    // 挿入されたタグを確認
    let tag_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tags")
        .fetch_one(pool)
        .await?;
    println!("    ℹ️  Total tags in database: {}", tag_count);

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
            "#,
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
            "#,
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

    // 商品配列 (重複を削除して整理)
    let products = [
        // シンプル商品（単一SKU） - (sku_code, name, color, material, dimensions, base_price, sale_price, stock, display_order)
        (
            "Lift - Walnut",
            "Monitor stand with storage space underneath",
            "monitor-risers",
            false,
            true,
            vec![(
                "LIFT-WALNUT-001",
                "Standard",
                "Walnut",
                "Walnut Wood",
                "24\" x 10\" x 6\"",
                39000,
                None::<i64>,
                20,
                1,
            )],
        ),
        (
            "Monument - Charcoal",
            "Phone stand with minimalist design",
            "accessories",
            false,
            true,
            vec![(
                "MONUMENT-CHARCOAL-001",
                "Standard",
                "Charcoal",
                "Metal",
                "4\" x 4\" x 6\"",
                9000,
                None::<i64>,
                35,
                1,
            )],
        ),
        (
            "Shelf - Black",
            "Wall-mounted shelf with modern black finish",
            "wall-shelves",
            false,
            true,
            vec![(
                "SHELF-BLACK-001",
                "Standard",
                "Black",
                "Metal",
                "24\" x 8\" x 2\"",
                11500,
                None::<i64>,
                25,
                1,
            )],
        ),
        (
            "Large Stand - Walnut",
            "Large monitor stand with walnut finish",
            "monitor-risers",
            false,
            false,
            vec![(
                "STAND-WALNUT-LARGE",
                "Large",
                "Walnut",
                "Walnut Wood",
                "30\" x 12\" x 8\"",
                15000,
                None::<i64>,
                15,
                1,
            )],
        ),
        (
            "Mini Shelf - White",
            "Compact wall shelf in white finish",
            "wall-shelves",
            false,
            true,
            vec![(
                "MINI-SHELF-WHITE",
                "Compact",
                "White",
                "Metal",
                "12\" x 6\" x 2\"",
                6000,
                None::<i64>,
                30,
                1,
            )],
        ),
        (
            "Table Light - Black",
            "Minimalist table lamp with adjustable brightness",
            "lighting",
            false,
            false,
            vec![(
                "TABLE-LIGHT-BLACK",
                "Standard",
                "Black",
                "Metal and LED",
                "6\" x 6\" x 18\"",
                32000,
                None::<i64>,
                18,
                1,
            )],
        ),
        (
            "Bench - Whitewash Oak",
            "Modern bench with whitewash oak finish",
            "bench-consoles",
            true,
            false,
            vec![(
                "BENCH-WHITEWASH-OAK",
                "Standard",
                "Whitewash Oak",
                "Whitewash Oak Wood",
                "48\" x 16\" x 18\"",
                230000,
                None::<i64>,
                8,
                1,
            )],
        ),
        (
            "Table - Black Oak",
            "Dining table with black oak finish",
            "tables",
            false,
            false,
            vec![(
                "TABLE-BLACK-OAK",
                "Standard",
                "Black Oak",
                "Black Oak Wood",
                "72\" x 36\" x 30\"",
                280000,
                None::<i64>,
                0,
                1,
            )],
        ),
        (
            "TUK - Black",
            "Premium bookshelf speakers with wireless connectivity",
            "audio",
            false,
            false,
            vec![(
                "TUK-BLACK-001",
                "Standard",
                "Black",
                "Wood and Metal",
                "8\" x 11\" x 13\"",
                80000,
                None::<i64>,
                12,
                1,
            )],
        ),
        (
            "Coffee Table - Walnut",
            "Modern coffee table with clean walnut finish",
            "tables",
            true,
            false,
            vec![(
                "COFFEE-TABLE-WALNUT",
                "Standard",
                "Walnut",
                "Walnut Wood",
                "40\" x 20\" x 16\"",
                120000,
                None::<i64>,
                10,
                1,
            )],
        ),
        (
            "Bookshelf - White Oak",
            "Five-tier bookshelf with white oak finish",
            "wall-shelves",
            false,
            false,
            vec![(
                "BOOKSHELF-WHITE-OAK",
                "Five-tier",
                "White Oak",
                "White Oak Wood",
                "32\" x 12\" x 72\"",
                89000,
                Some(75000),
                14,
                1,
            )],
        ),
        (
            "Pendant Light - Brass",
            "Modern pendant light with brass finish",
            "lighting",
            false,
            true,
            vec![(
                "PENDANT-BRASS-001",
                "Standard",
                "Brass",
                "Brass and Glass",
                "8\" x 8\" x 12\"",
                24000,
                None::<i64>,
                22,
                1,
            )],
        ),
        (
            "Side Table - Black Oak",
            "Compact side table with black oak finish",
            "side-tables",
            false,
            true,
            vec![(
                "SIDE-TABLE-BLACK-OAK",
                "Compact",
                "Black Oak",
                "Black Oak Wood",
                "18\" x 18\" x 24\"",
                38000,
                None::<i64>,
                16,
                1,
            )],
        ),
        (
            "Floor Lamp - White",
            "Minimalist floor lamp with adjustable head",
            "lighting",
            false,
            false,
            vec![(
                "FLOOR-LAMP-WHITE",
                "Adjustable",
                "White",
                "Metal and Fabric",
                "12\" x 12\" x 60\"",
                45000,
                None::<i64>,
                10,
                1,
            )],
        ),
        (
            "Wireless Charger - Oak",
            "Wireless charging pad with oak veneer",
            "accessories",
            false,
            true,
            vec![(
                "WIRELESS-CHARGER-OAK",
                "Standard",
                "White Oak",
                "Oak Veneer and Electronics",
                "4\" x 4\" x 0.5\"",
                12000,
                None::<i64>,
                40,
                1,
            )],
        ),
        (
            "Storage Bench - Gray",
            "Storage bench with soft gray upholstery",
            "bench-consoles",
            false,
            false,
            vec![(
                "STORAGE-BENCH-GRAY",
                "With Storage",
                "Gray",
                "Fabric and Wood Frame",
                "36\" x 16\" x 18\"",
                56000,
                None::<i64>,
                12,
                1,
            )],
        ),
        (
            "Wall Clock - Black",
            "Minimalist wall clock with black frame",
            "accessories",
            false,
            true,
            vec![(
                "WALL-CLOCK-BLACK",
                "Standard",
                "Black",
                "Metal and Glass",
                "12\" x 12\" x 2\"",
                8500,
                None::<i64>,
                28,
                1,
            )],
        ),
        // 複数SKU商品 - より戦略的なdisplay_order設定
        (
            "Desk - Walnut",
            "Minimalist walnut desk with clean lines and modern design",
            "desks",
            true,
            false,
            vec![
                (
                    "DESK-WALNUT-SMALL",
                    "Small",
                    "Walnut",
                    "Walnut Wood",
                    "48\" x 24\" x 30\"",
                    179000,
                    Some(160000),
                    15,
                    1,
                ), // 人気サイズを優先
                (
                    "DESK-WALNUT-LARGE",
                    "Large",
                    "Walnut",
                    "Walnut Wood",
                    "48\" x 24\" x 30\"",
                    229000,
                    Some(179000),
                    8,
                    2,
                ),
            ],
        ),
        (
            "Form Armchair Swivel - Upholstered",
            "Comfortable swivel armchair with premium upholstery",
            "seating",
            false,
            false,
            vec![
                (
                    "FORM-CHAIR-BLACK",
                    "Black Upholstery",
                    "Black",
                    "Fabric and Metal",
                    "28\" x 28\" x 32\"",
                    134500,
                    Some(99999),
                    6,
                    1,
                ), // より手頃な価格を優先
                (
                    "FORM-CHAIR-GRAY",
                    "Gray Upholstery",
                    "Gray",
                    "Fabric and Metal",
                    "28\" x 28\" x 32\"",
                    171500,
                    Some(99999),
                    4,
                    2,
                ),
            ],
        ),
        (
            "Desk - White Oak",
            "Modern desk with white oak finish",
            "desks",
            false,
            false,
            vec![
                (
                    "DESK-WHITE-OAK-SMALL",
                    "Small",
                    "White Oak",
                    "White Oak Wood",
                    "48\" x 24\" x 30\"",
                    179000,
                    Some(159000),
                    12,
                    1,
                ), // 人気サイズを優先
                (
                    "DESK-WHITE-OAK-LARGE",
                    "Large",
                    "White Oak",
                    "White Oak Wood",
                    "48\" x 24\" x 30\"",
                    219000,
                    Some(179000),
                    7,
                    2,
                ),
            ],
        ),
        (
            "Office Chair - Black",
            "Ergonomic office chair with leather upholstery",
            "seating",
            false,
            false,
            vec![
                (
                    "OFFICE-CHAIR-BLACK-STD",
                    "Standard Height",
                    "Black",
                    "Leather and Metal",
                    "26\" x 26\" x 42\"",
                    68000,
                    None::<i64>,
                    10,
                    1,
                ), // 標準サイズを優先
                (
                    "OFFICE-CHAIR-BLACK-TALL",
                    "Tall Height",
                    "Black",
                    "Leather and Metal",
                    "26\" x 26\" x 42\"",
                    72000,
                    None::<i64>,
                    8,
                    2,
                ),
            ],
        ),
        (
            "Dining Chair - Walnut",
            "Modern dining chair with walnut frame",
            "seating",
            true,
            false,
            vec![
                (
                    "DINING-CHAIR-WALNUT-BEIGE",
                    "Beige Cushion",
                    "Beige",
                    "Walnut Wood and Fabric",
                    "18\" x 20\" x 32\"",
                    32000,
                    None::<i64>,
                    20,
                    1,
                ), // ベージュを優先（汎用性）
                (
                    "DINING-CHAIR-WALNUT-GRAY",
                    "Gray Cushion",
                    "Gray",
                    "Walnut Wood and Fabric",
                    "18\" x 20\" x 32\"",
                    34000,
                    None::<i64>,
                    15,
                    2,
                ),
            ],
        ),
    ];

    let mut product_index = 0;
    for (name, description, category_slug, is_best_seller, is_quick_ship, skus) in products.iter() {
        // カテゴリーIDを取得
        let category_id: String = sqlx::query_scalar("SELECT id FROM categories WHERE slug = ?")
            .bind(category_slug)
            .fetch_one(pool)
            .await?;

        // 商品IDを生成
        let product_id = Uuid::new_v4().to_string();

        // 商品を挿入
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO products (
                id, name, description, category_id,
                is_best_seller, is_quick_ship
            )
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
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
        for i in 0..2 {
            sqlx::query(
                r#"
                INSERT OR IGNORE INTO product_images (
                    product_id, image_url, alt_text, display_order
                )
                VALUES (?, ?, ?, ?)
                "#,
            )
            .bind(&product_id)
            .bind(format!(
                "https://picsum.photos/id/{}/800/800",
                product_index + i
            ))
            .bind(format!("Image {} of {}", i + 1, name))
            .bind(i as i64)
            .execute(pool)
            .await?;
        }

        println!("  ✓ Product created: {} (ID: {})", name, product_id);

        // 各SKUを挿入
        let mut sku_index = 0;
        for (
            sku_code,
            sku_name,
            color_name,
            material,
            dimensions,
            base_price,
            sale_price,
            stock_quantity,
            display_order,
        ) in skus
        {
            // 色IDを取得（エラーハンドリングを改善）
            let color_id: Option<i64> = sqlx::query_scalar("SELECT id FROM colors WHERE name = ?")
                .bind(color_name)
                .fetch_optional(pool)
                .await?;

            let color_id = match color_id {
                Some(id) => id,
                None => {
                    println!(
                        "    ⚠️  Warning: Color '{}' not found, skipping SKU {}",
                        color_name, sku_code
                    );
                    continue;
                }
            };

            // SKUを挿入
            let sku_id = Uuid::new_v4().to_string();

            sqlx::query(
                r#"
                INSERT OR IGNORE INTO skus (
                    id, product_id, sku_code, name,
                    color_id, material, dimensions,
                    base_price, sale_price, 
                    stock_quantity, reserved_quantity, low_stock_threshold,
                    display_order, image_url
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, 5, ?, ?)
                "#,
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
            .bind(display_order)
            .bind(format!(
                "https://picsum.photos/id/{}/800/800",
                product_index + 5 + sku_index
            ))
            .execute(pool)
            .await?;
            sku_index += 1;
            println!("    ↳ SKU created: {} (ID: {})", sku_name, sku_id);
        }
        product_index += 20;

        // 商品に対するタグを挿入
        if *is_best_seller {
            if let Some(best_seller_tag_id) =
                sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE slug = 'best_seller'")
                    .fetch_optional(pool)
                    .await?
            {
                sqlx::query("INSERT INTO product_tags (product_id, tag_id) VALUES (?, ?)")
                    .bind(&product_id)
                    .bind(best_seller_tag_id)
                    .execute(pool)
                    .await?;
                println!("    ↳ Best seller tag added");
            }
        }

        if *is_quick_ship {
            if let Some(quick_ship_tag_id) =
                sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE slug = 'quick_ship'")
                    .fetch_optional(pool)
                    .await?
            {
                sqlx::query("INSERT INTO product_tags (product_id, tag_id) VALUES (?, ?)")
                    .bind(&product_id)
                    .bind(quick_ship_tag_id)
                    .execute(pool)
                    .await?;
                println!("    ↳ Quick ship tag added");
            }
        }

        // 売り切れ商品にタグを追加
        if *name == "Table - Black Oak" {
            if let Some(sold_out_tag_id) =
                sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE slug = 'sold_out'")
                    .fetch_optional(pool)
                    .await?
            {
                sqlx::query("INSERT INTO product_tags (product_id, tag_id) VALUES (?, ?)")
                    .bind(&product_id)
                    .bind(sold_out_tag_id)
                    .execute(pool)
                    .await?;
                println!("    ↳ Sold out tag added");
            }
        }

        // セール商品にタグを追加（sale_priceが設定されている商品）
        let has_sale = skus
            .iter()
            .any(|(_, _, _, _, _, _, sale_price, _, _)| sale_price.is_some());
        if has_sale {
            if let Some(on_sale_tag_id) =
                sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE slug = 'on_sale'")
                    .fetch_optional(pool)
                    .await?
            {
                sqlx::query("INSERT INTO product_tags (product_id, tag_id) VALUES (?, ?)")
                    .bind(&product_id)
                    .bind(on_sale_tag_id)
                    .execute(pool)
                    .await?;
                println!("    ↳ On sale tag added");
            }
        }
    }

    println!("✅ Sample products seeded successfully!");
    Ok(())
}

/// サンプルクーポンを挿入
pub async fn seed_sample_coupons() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();

    println!("🎫 Seeding sample coupons...");

    let coupons = [
        (
            "550e8400-e29b-41d4-a716-446655440001",
            "WELCOME10",
            "新規顧客10%オフ",
            Some("初回購入時に10%割引"),
            "percentage",
            10,
            Some(5000),
            Some(100),
            "2024-01-01",
            "2044-12-31",
        ),
        (
            "550e8400-e29b-41d4-a716-446655440002",
            "SAVE20",
            "20%オフクーポン",
            Some("全商品20%割引"),
            "percentage",
            20,
            Some(10000),
            Some(50),
            "2024-01-01",
            "2044-12-31",
        ),
        (
            "550e8400-e29b-41d4-a716-446655440003",
            "FLAT1000",
            "1000円割引",
            Some("1000円固定割引"),
            "fixed_amount",
            1000,
            Some(3000),
            Some(200),
            "2024-01-01",
            "2044-12-31",
        ),
    ];

    for (id, code, name, description, discount_type, discount_value, minimum_amount, usage_limit, valid_from, valid_until) in coupons {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO coupons (
                id, code, name, description, discount_type, discount_value, 
                minimum_amount, usage_limit, valid_from, valid_until
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id)
        .bind(code)
        .bind(name)
        .bind(description)
        .bind(discount_type)
        .bind(discount_value)
        .bind(minimum_amount)
        .bind(usage_limit)
        .bind(valid_from)
        .bind(valid_until)
        .execute(pool)
        .await?;

        println!("  ✓ Coupon created: {}", code);
    }

    println!("✅ Sample coupons seeded successfully!");
    Ok(())
}
