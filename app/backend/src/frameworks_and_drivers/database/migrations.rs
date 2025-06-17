use anyhow::Result;
use sqlx::SqlitePool;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    // データベース接続を直接作成
    let pool = SqlitePool::connect(database_url).await?;
    
    // Phase 2: 正規化スキーマ（カテゴリー、色、タグなど）
    create_normalized_schema(&pool).await?;
    
    // Phase 1: 新しい商品テーブル作成（カテゴリー依存のため後に実行）
    create_basic_products_table(&pool).await?;
    
    println!("✅ All migrations completed successfully!");
    Ok(())
}

/// Phase 1: 新しい商品テーブル作成
async fn create_basic_products_table(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            material TEXT,
            dimensions TEXT,
            base_price INTEGER NOT NULL,
            sale_price INTEGER,
            category_id INTEGER NOT NULL,
            quantity INTEGER NOT NULL DEFAULT 0,
            is_active BOOLEAN DEFAULT TRUE,
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
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_category_id ON products(category_id)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_name ON products(name)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_is_active ON products(is_active)")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_flags ON products(is_best_seller, is_quick_ship)")
        .execute(pool).await?;
    
    println!("📦 Enhanced products table created");
    Ok(())
}

/// Phase 2: 正規化スキーマ作成
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
            is_active BOOLEAN DEFAULT TRUE,
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

    // 色テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS colors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            hex_code TEXT,
            display_order INTEGER DEFAULT 0,
            is_active BOOLEAN DEFAULT TRUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_colors_name ON colors(name)")
        .execute(pool).await?;



    // 商品画像テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            image_url TEXT NOT NULL,
            alt_text TEXT,
            is_main BOOLEAN DEFAULT FALSE,
            sort_order INTEGER DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id)")
        .execute(pool).await?;

    // 商品色関連テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_colors (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            color_id INTEGER NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE CASCADE,
            UNIQUE(product_id, color_id)
        )
        "#
    )
    .execute(pool)
    .await?;

    // 商品バリアントテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_variants (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            base_price INTEGER NOT NULL,
            sale_price INTEGER,
            image_url TEXT,
            quantity INTEGER NOT NULL DEFAULT 0,
            is_available BOOLEAN DEFAULT TRUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE RESTRICT
        )
        "#
    )
    .execute(pool)
    .await?;

    // タグテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            color_code TEXT DEFAULT '#666666',
            priority INTEGER DEFAULT 0,
            is_system BOOLEAN DEFAULT FALSE,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#
    )
    .execute(pool)
    .await?;

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

    // 在庫管理テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS inventory (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id INTEGER,
            variant_id INTEGER,
            total_quantity INTEGER NOT NULL DEFAULT 0,
            reserved_quantity INTEGER NOT NULL DEFAULT 0,
            low_stock_threshold INTEGER DEFAULT 10,
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (variant_id) REFERENCES product_variants(id) ON DELETE CASCADE,
            CHECK (
                (product_id IS NOT NULL AND variant_id IS NULL) OR 
                (product_id IS NULL AND variant_id IS NOT NULL)
            ),
            CHECK (reserved_quantity <= total_quantity),
            CHECK (total_quantity >= 0),
            CHECK (reserved_quantity >= 0)
        )
        "#
    )
    .execute(pool)
    .await?;

    println!("🏗️  Normalized schema created");
    Ok(())
}

 