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

    // Phase 4: 配送方法テーブル作成
    create_shipping_methods_table(&pool).await?;

    // Phase 5: 支払い方法テーブル作成
    create_payment_methods_table(&pool).await?;

    // Phase 6: 注文関連テーブル作成
    create_order_tables(&pool).await?;

    // Phase 7: クーポンテーブル作成
    create_coupon_tables(&pool).await?;

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
        "#,
    )
    .execute(pool)
    .await?;

    // カテゴリーインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_categories_parent_id ON categories(parent_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_categories_slug ON categories(slug)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_categories_display_order ON categories(display_order)",
    )
    .execute(pool)
    .await?;

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
        "#,
    )
    .execute(pool)
    .await?;

    // 色インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_colors_name ON colors(name)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_colors_hex ON colors(hex)")
        .execute(pool)
        .await?;

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
        "#,
    )
    .execute(pool)
    .await?;

    // タグインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_slug ON tags(slug)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_tags_priority ON tags(priority)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_tags_system ON tags(is_system) WHERE is_system = 1",
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
        "#,
    )
    .execute(pool)
    .await?;

    // 商品インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id)")
        .execute(pool)
        .await?;
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
            display_order INTEGER NOT NULL DEFAULT 0,
            image_url TEXT,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
            FOREIGN KEY (color_id) REFERENCES colors(id) ON DELETE RESTRICT,
            CONSTRAINT positive_prices CHECK (base_price >= 0),
            CONSTRAINT positive_stock CHECK (stock_quantity >= 0),
            CONSTRAINT valid_reserved CHECK (reserved_quantity <= stock_quantity),
            CONSTRAINT positive_display_order CHECK (display_order >= 0)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // SKUインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_product_id ON skus(product_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_code ON skus(sku_code)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_color ON skus(color_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_dimensions ON skus(dimensions) WHERE dimensions IS NOT NULL")
        .execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_skus_material ON skus(material) WHERE material IS NOT NULL",
    )
    .execute(pool)
    .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_skus_stock ON skus(stock_quantity, reserved_quantity)",
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_price ON skus(base_price, sale_price)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_skus_low_stock ON skus(stock_quantity, reserved_quantity, low_stock_threshold) WHERE stock_quantity - reserved_quantity <= low_stock_threshold AND stock_quantity - reserved_quantity > 0")
        .execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_skus_display_order ON skus(product_id, display_order)",
    )
    .execute(pool)
    .await?;

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
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_product_images_product_id ON product_images(product_id)",
    )
    .execute(pool)
    .await?;
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
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_product_tags_product_id ON product_tags(product_id)",
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_product_tags_tag_id ON product_tags(tag_id)")
        .execute(pool)
        .await?;

    println!("🔗 Product related tables created (skus, images, product_tags)");
    Ok(())
}

/// Phase 4: 配送方法テーブル作成
async fn create_shipping_methods_table(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shipping_methods (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            price INTEGER NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            CONSTRAINT positive_price CHECK (price >= 0),
            CONSTRAINT positive_sort_order CHECK (sort_order >= 0)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 配送方法インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_shipping_methods_active ON shipping_methods(is_active) WHERE is_active = 1")
        .execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_shipping_methods_sort_order ON shipping_methods(sort_order)")
        .execute(pool).await?;

    // 初期データ挿入
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO shipping_methods (id, name, description, price, sort_order) VALUES
        ('standard', '標準配送', '5-7営業日', 500, 1),
        ('express', '速達配送', '2-3営業日', 1000, 2),
        ('overnight', '翌日配送', '翌営業日', 2000, 3)
        "#,
    )
    .execute(pool)
    .await?;

    println!("🚚 Shipping methods table created with initial data");
    Ok(())
}

/// Phase 5: 支払い方法テーブル作成
async fn create_payment_methods_table(pool: &sqlx::SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS payment_methods (
            id TEXT PRIMARY KEY NOT NULL,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            is_active BOOLEAN NOT NULL DEFAULT TRUE,
            sort_order INTEGER NOT NULL DEFAULT 0,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            CONSTRAINT positive_sort_order CHECK (sort_order >= 0)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 支払い方法インデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_payment_methods_active ON payment_methods(is_active) WHERE is_active = 1")
        .execute(pool).await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_payment_methods_sort_order ON payment_methods(sort_order)",
    )
    .execute(pool)
    .await?;

    // 初期データ挿入
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO payment_methods (id, name, description, is_active, sort_order) VALUES
        ('credit_card', 'クレジットカード', 'VISA、MasterCard、JCB対応', 1, 1),
        ('cod', '代引き', '商品到着時に現金でお支払い', 1, 2),
        ('bank_transfer', '銀行振込', '指定口座への事前振込', 1, 3),
        ('convenience_store', 'コンビニ支払い', 'セブンイレブン、ファミリーマート等', 1, 4)
        "#,
    )
    .execute(pool)
    .await?;

    println!("💳 Payment methods table created with initial data");
    Ok(())
}

/// Phase 6: 注文関連テーブル作成
async fn create_order_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    // 注文テーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS orders (
            id TEXT PRIMARY KEY,
            order_number TEXT UNIQUE NOT NULL,
            
            -- 顧客情報
            customer_first_name TEXT NOT NULL,
            customer_last_name TEXT NOT NULL,
            customer_email TEXT NOT NULL,
            customer_phone TEXT NOT NULL,
            
            -- 配送情報
            shipping_method_id TEXT NOT NULL,
            shipping_fee INTEGER NOT NULL,
            shipping_postal_code TEXT NOT NULL,
            shipping_prefecture TEXT NOT NULL,
            shipping_city TEXT NOT NULL,
            shipping_street TEXT NOT NULL,
            shipping_building TEXT,
            
            -- 支払い情報
            payment_method_id TEXT NOT NULL,
            payment_fee INTEGER NOT NULL,
            payment_details TEXT,
            
            -- 価格情報
            subtotal INTEGER NOT NULL,
            shipping_fee_total INTEGER NOT NULL,
            payment_fee_total INTEGER NOT NULL,
            tax_amount INTEGER NOT NULL,
            total_amount INTEGER NOT NULL,
            
            -- ステータスとタイムスタンプ
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            paid_at TEXT,
            shipped_at TEXT,
            delivered_at TEXT,
            cancelled_at TEXT,
            
            -- オプション
            delivery_info_id TEXT,
            notes TEXT,
            
            FOREIGN KEY (shipping_method_id) REFERENCES shipping_methods(id),
            FOREIGN KEY (payment_method_id) REFERENCES payment_methods(id),
            CONSTRAINT valid_status CHECK (status IN ('pending', 'paid', 'processing', 'shipped', 'delivered', 'cancelled', 'refunded')),
            CONSTRAINT positive_amounts CHECK (subtotal >= 0 AND tax_amount >= 0 AND total_amount >= 0)
        )
        "#
    )
    .execute(pool)
    .await?;

    // 注文アイテムテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS order_items (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id TEXT NOT NULL,
            
            -- SKU情報
            sku_id TEXT NOT NULL,
            sku_code TEXT NOT NULL,
            product_name TEXT NOT NULL,
            sku_name TEXT NOT NULL,
            
            -- 価格情報
            unit_price INTEGER NOT NULL,
            quantity INTEGER NOT NULL,
            subtotal INTEGER NOT NULL,
            
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            
            FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
            FOREIGN KEY (sku_id) REFERENCES skus(id),
            CONSTRAINT positive_quantity CHECK (quantity > 0),
            CONSTRAINT positive_price CHECK (unit_price >= 0),
            CONSTRAINT valid_subtotal CHECK (subtotal = unit_price * quantity)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // 注文イベントテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS order_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            order_id TEXT NOT NULL,
            event_type TEXT NOT NULL,
            event_data TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            
            FOREIGN KEY (order_id) REFERENCES orders(id) ON DELETE CASCADE,
            CONSTRAINT valid_event_type CHECK (event_type IN (
                'order_created', 'order_paid', 'order_shipped', 
                'order_delivered', 'order_cancelled', 'order_refunded'
            ))
        )
        "#,
    )
    .execute(pool)
    .await?;

    // インデックス作成
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_customer_email ON orders(customer_email)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_order_number ON orders(order_number)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_order_items_sku_id ON order_items(sku_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_order_events_order_id ON order_events(order_id)")
        .execute(pool)
        .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_order_events_type ON order_events(event_type)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_order_events_created_at ON order_events(created_at)",
    )
    .execute(pool)
    .await?;

    println!("📦 Order tables created (orders, order_items, order_events)");
    Ok(())
}

/// Phase 7: クーポンテーブル作成
async fn create_coupon_tables(pool: &sqlx::SqlitePool) -> Result<()> {
    // クーポンテーブル
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS coupons (
            id TEXT PRIMARY KEY NOT NULL,
            code TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL,
            description TEXT,
            discount_type TEXT NOT NULL,
            discount_value INTEGER NOT NULL,
            minimum_amount INTEGER,
            usage_limit INTEGER,
            used_count INTEGER NOT NULL DEFAULT 0,
            valid_from TEXT NOT NULL,
            valid_until TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            updated_at TEXT NOT NULL DEFAULT (datetime('now')),
            
            CONSTRAINT valid_discount_type CHECK (discount_type IN ('percentage', 'fixed_amount')),
            CONSTRAINT positive_discount_value CHECK (discount_value > 0),
            CONSTRAINT positive_minimum_amount CHECK (minimum_amount IS NULL OR minimum_amount >= 0),
            CONSTRAINT positive_usage_limit CHECK (usage_limit IS NULL OR usage_limit > 0),
            CONSTRAINT positive_used_count CHECK (used_count >= 0)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // クーポンインデックス
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_coupons_code ON coupons(code)")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_coupons_validity ON coupons(valid_from, valid_until)",
    )
    .execute(pool)
    .await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_coupons_usage ON coupons(usage_limit, used_count)")
        .execute(pool)
        .await?;

    // 初期データ挿入（テスト用）
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO coupons (
            id, code, name, description, discount_type, discount_value, 
            minimum_amount, usage_limit, valid_from, valid_until
        ) VALUES
        ('550e8400-e29b-41d4-a716-446655440001', 'WELCOME10', '新規顧客10%オフ', '初回購入時に10%割引', 'percentage', 10, 5000, 100, '2024-01-01', '2044-12-31'),
        ('550e8400-e29b-41d4-a716-446655440002', 'SAVE20', '20%オフクーポン', '全商品20%割引', 'percentage', 20, 10000, 50, '2024-01-01', '2044-12-31'),
        ('550e8400-e29b-41d4-a716-446655440003', 'FLAT1000', '1000円割引', '1000円固定割引', 'fixed_amount', 1000, 3000, 200, '2024-01-01', '2044-12-31')
        "#,
    )
    .execute(pool)
    .await?;

    println!("🎫 Coupon tables created with initial test data");
    Ok(())
}
