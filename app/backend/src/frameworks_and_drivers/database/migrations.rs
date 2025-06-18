use anyhow::Result;
use sqlx::SqlitePool;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    // データベース接続を直接作成
    let pool = SqlitePool::connect(database_url).await?;
    
    // Phase 1: 正規化スキーマ（カテゴリー、色、タグなど）
    create_normalized_schema(&pool).await?;
    
    // Phase 2: 商品テーブル作成（カテゴリーと色テーブルに依存）
    create_products_table(&pool).await?;
    
    // Phase 3: 商品関連テーブル作成（商品テーブルに依存）
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
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            slug TEXT NOT NULL UNIQUE,
            parent_id INTEGER,
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

    println!("🏗️  Normalized schema created (categories, colors, tags)");
    Ok(())
}

/// Phase 2: 商品テーブル作成
async fn create_products_table(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            sku TEXT UNIQUE,
            description TEXT,
            material TEXT,
            dimensions TEXT,
            color_id INTEGER,
            category_id INTEGER NOT NULL,
            base_price INTEGER,
            sale_price INTEGER,
            stock_quantity INTEGER DEFAULT 0,
            reserved_quantity INTEGER DEFAULT 0,
            low_stock_threshold INTEGER DEFAULT 5,
            has_variants BOOLEAN DEFAULT FALSE,
            is_active BOOLEAN DEFAULT TRUE,
            is_best_seller BOOLEAN DEFAULT FALSE,
            is_quick_ship BOOLEAN DEFAULT FALSE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE RESTRICT,
            CONSTRAINT positive_prices CHECK (base_price IS NULL OR base_price >= 0),
            CONSTRAINT positive_stock CHECK (stock_quantity >= 0),
            CONSTRAINT valid_reserved CHECK (reserved_quantity <= stock_quantity),
            CONSTRAINT price_consistency CHECK (
                (has_variants = FALSE AND base_price IS NOT NULL) OR
                (has_variants = TRUE AND base_price IS NULL)
            ),
            CONSTRAINT color_consistency CHECK (
                (has_variants = FALSE AND color_id IS NOT NULL) OR
                (has_variants = TRUE AND color_id IS NULL)
            )
        )
        "#
    )
    .execute(pool)
    .await?;

    // 商品インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_category_active ON products(category_id, is_active)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_has_variants ON products(has_variants)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_color ON products(color_id) WHERE has_variants = FALSE")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_price_range ON products(base_price, sale_price) WHERE has_variants = FALSE")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_stock ON products(stock_quantity, reserved_quantity) WHERE has_variants = FALSE")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_flags ON products(is_best_seller, is_quick_ship) WHERE is_active = TRUE")
        .execute(pool).await?;
    
    println!("📦 Products table created with constraints and indexes");
    Ok(())
}

/// Phase 3: 商品関連テーブル作成
async fn create_product_related_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    // 商品画像テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            image_url TEXT NOT NULL,
            alt_text TEXT,
            is_main BOOLEAN DEFAULT FALSE,
            display_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id)")
        .execute(pool).await?;

    // 商品バリアントテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_variants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            sku TEXT UNIQUE,
            name TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            size TEXT,
            base_price INTEGER NOT NULL,
            sale_price INTEGER,
            cost_price INTEGER,
            stock_quantity INTEGER DEFAULT 0,
            reserved_quantity INTEGER DEFAULT 0,
            low_stock_threshold INTEGER DEFAULT 5,
            is_available BOOLEAN DEFAULT TRUE,
            image_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE RESTRICT,
            CONSTRAINT positive_variant_prices CHECK (base_price >= 0),
            CONSTRAINT positive_variant_stock CHECK (stock_quantity >= 0),
            CONSTRAINT valid_variant_reserved CHECK (reserved_quantity <= stock_quantity)
        )
        "#
    )
    .execute(pool)
    .await?;

    // バリアントインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_product_id ON product_variants(product_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_sku ON product_variants(sku) WHERE sku IS NOT NULL")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_available ON product_variants(product_id, is_available)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_color ON product_variants(color_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_price ON product_variants(base_price, sale_price)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_variants_stock ON product_variants(stock_quantity, reserved_quantity)")
        .execute(pool).await?;

    // 商品タグ関連テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
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

    println!("🔗 Product related tables created (images, variants, tags)");
    Ok(())
}

 