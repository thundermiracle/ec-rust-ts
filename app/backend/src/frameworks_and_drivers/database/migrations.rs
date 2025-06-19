use anyhow::Result;
use sqlx::SqlitePool;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    // データベース接続を直接作成
    let pool = SqlitePool::connect(database_url).await?;
    
    // Phase 1: 正規化スキーマ（カテゴリー、色、タグなど）
    create_normalized_schema(&pool).await?;
    
    // Phase 2: 商品テーブル作成（カテゴリーと色テーブルに依存）
    create_products_table(&pool).await?;
    
    // Phase 3: SKUと商品関連テーブル作成（商品テーブルに依存）
    create_product_related_tables(&pool).await?;
    
    println!("✅ All migrations completed successfully!");
    Ok(())
}

/// Phase 1: 正規化スキーマ作成（カテゴリー、色、タグ）
async fn create_normalized_schema(pool: &sqlx::SqlitePool) -> Result<()> {
    // カテゴリーテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL UNIQUE,
            slug TEXT NOT NULL UNIQUE,
            parent_id TEXT,
            display_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    // カテゴリーインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_categories_display_order ON categories(display_order)")
        .execute(pool).await?;

    // 色テーブル - 中央集権的な色マスターテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS colors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            hex TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#
    )
    .execute(pool)
    .await?;

    // 色インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_colors_name ON colors(name)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_colors_hex ON colors(hex)")
        .execute(pool).await?;

    // タグテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            priority INTEGER DEFAULT 0,
            is_system BOOLEAN DEFAULT FALSE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#
    )
    .execute(pool)
    .await?;

    // タグインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags(slug)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_priority ON tags(priority)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_system ON tags(is_system) WHERE is_system = 1")
        .execute(pool).await?;

    println!("🏗️  Normalized schema created (categories, colors, tags)");
    Ok(())
}

/// Phase 2: 商品テーブル作成
async fn create_products_table(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT,
            category_id TEXT NOT NULL,
            is_best_seller BOOLEAN DEFAULT FALSE,
            is_quick_ship BOOLEAN DEFAULT FALSE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT
        )
        "#
    )
    .execute(pool)
    .await?;

    // 商品インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_best_seller ON products(is_best_seller) WHERE is_best_seller = 1")
        .execute(pool).await?;
    
    println!("📦 Products table created with constraints and indexes");
    Ok(())
}

/// Phase 3: SKUと商品関連テーブル作成
async fn create_product_related_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    // SKU（Stock Keeping Unit）テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS skus (
            id TEXT PRIMARY KEY NOT NULL,
            product_id TEXT NOT NULL,
            sku_code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            dimensions TEXT,
            material TEXT,
            base_price INTEGER NOT NULL,
            sale_price INTEGER,
            stock_quantity INTEGER DEFAULT 0,
            reserved_quantity INTEGER DEFAULT 0,
            low_stock_threshold INTEGER DEFAULT 5,
            image_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE RESTRICT,
            CONSTRAINT positive_prices CHECK (base_price >= 0),
            CONSTRAINT positive_stock CHECK (stock_quantity >= 0),
            CONSTRAINT valid_reserved CHECK (reserved_quantity <= stock_quantity)
        )
        "#
    )
    .execute(pool)
    .await?;

    // SKUインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_product_id ON skus(product_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_code ON skus(sku_code)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_color ON skus(color_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_dimensions ON skus(dimensions) WHERE dimensions IS NOT NULL")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_material ON skus(material) WHERE material IS NOT NULL")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_stock ON skus(stock_quantity, reserved_quantity)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_price ON skus(base_price, sale_price)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_low_stock ON skus(stock_quantity, reserved_quantity, low_stock_threshold) WHERE stock_quantity - reserved_quantity <= low_stock_threshold AND stock_quantity - reserved_quantity > 0")
        .execute(pool).await?;

    // 商品画像テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id TEXT NOT NULL,
            image_url TEXT NOT NULL,
            alt_text TEXT,
            display_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_images_order ON product_images(product_id, display_order)")
        .execute(pool).await?;

    // 商品タグ関連テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id TEXT NOT NULL,
            tag_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE,
            UNIQUE(product_id, tag_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_tags_product_id ON product_tags(product_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_tags_tag_id ON product_tags(tag_id)")
        .execute(pool).await?;

    println!("🔗 Product related tables created (skus, images, product_tags)");
    Ok(())
}

 