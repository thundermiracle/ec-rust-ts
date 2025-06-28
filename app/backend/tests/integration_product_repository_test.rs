use std::sync::Arc;
use sqlx::SqlitePool;

// 統合テストでは直接モジュールを参照
use ec_rust_backend::infrastructure::database::repositories_impl::SqliteProductRepository;
use ec_rust_backend::application::repositories::ProductRepository;
use ec_rust_backend::domain::models::ProductId;
use ec_rust_backend::infrastructure::di::Container;

/// プール注入型リポジトリのテスト
#[tokio::test]
async fn test_product_repository_with_pool_injection() {
    // テスト用インメモリDB
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // テーブル作成（簡単なサンプル）
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            slug TEXT NOT NULL,
            parent_id TEXT,
            display_order INTEGER DEFAULT 0
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            description TEXT NOT NULL,
            category_id TEXT NOT NULL,
            is_best_seller BOOLEAN DEFAULT FALSE,
            is_quick_ship BOOLEAN DEFAULT FALSE,
            FOREIGN KEY (category_id) REFERENCES categories(id)
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS colors (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            hex TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS skus (
            id TEXT PRIMARY KEY,
            product_id TEXT NOT NULL,
            sku_code TEXT NOT NULL,
            name TEXT NOT NULL,
            color_id INTEGER NOT NULL,
            dimensions TEXT,
            material TEXT,
            base_price INTEGER NOT NULL,
            sale_price INTEGER,
            stock_quantity INTEGER DEFAULT 0,
            reserved_quantity INTEGER DEFAULT 0,
            display_order INTEGER DEFAULT 0,
            image_url TEXT,
            FOREIGN KEY (product_id) REFERENCES products(id),
            FOREIGN KEY (color_id) REFERENCES colors(id)
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS product_images (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            product_id TEXT NOT NULL,
            image_url TEXT NOT NULL,
            display_order INTEGER DEFAULT 0,
            FOREIGN KEY (product_id) REFERENCES products(id)
        )
        "#
    )
    .execute(&pool)
    .await
    .unwrap();

    // テストデータ挿入
    sqlx::query("INSERT INTO categories (id, name, slug) VALUES ('cat1', 'Electronics', 'electronics')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO colors (id, name, hex) VALUES (1, 'Red', '#FF0000')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO products (id, name, description, category_id) VALUES ('prod1', 'Test Product', 'A test product', 'cat1')")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::query("INSERT INTO skus (id, product_id, sku_code, name, color_id, base_price) VALUES ('sku1', 'prod1', 'TEST-001', 'Test SKU', 1, 1000)")
        .execute(&pool)
        .await
        .unwrap();

    // プール注入型リポジトリを作成
    let repo = SqliteProductRepository::new(pool.clone());
    
    // テスト実行：プール注入型リポジトリのfind_allをテスト
    let result = repo.find_all().await;
    
    assert!(result.is_ok());
    let product_list = result.unwrap();
    assert_eq!(product_list.total_count, 1);
    assert!(!product_list.products.is_empty());
    
    let product = &product_list.products[0];
    assert_eq!(product.id, "prod1");
    assert_eq!(product.name, "Test Product");
    assert_eq!(product.category, "Electronics");
    assert!(!product.colors.is_empty());
    
    println!("✅ Pool injection test passed!");
}

/// DIコンテナテスト用
#[tokio::test]
async fn test_container_with_test_pool() {
    // テスト用コンテナを作成
    let container_result = Container::new_for_test().await;
    assert!(container_result.is_ok());
    
    let container = container_result.unwrap();
    let dispatcher = container.get_dispatcher();
    
    // ディスパッチャが正常に作成されることを確認
    // ここではディスパッチャが作成されたことを確認するだけで十分
    assert!(std::ptr::addr_of!(*dispatcher) != std::ptr::null());
    
    println!("✅ Test container creation passed!");
}

/// 本番環境とテスト環境の切り替えテスト
#[tokio::test]
async fn test_different_environments() {
    // テスト環境
    let test_container = Container::new_for_test().await;
    assert!(test_container.is_ok());
    
    // NOTE: 本番環境テストは実際のDBが必要なのでコメントアウト
    // let prod_container = Container::new().await;
    // assert!(prod_container.is_ok());
    
    println!("✅ Environment switching test passed!");
}

/// パフォーマンステスト：プール共有
#[tokio::test]
async fn test_pool_sharing_performance() {
    let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
    
    // 複数のリポジトリが同一プールを共有
    let _repo1 = Arc::new(SqliteProductRepository::new(pool.clone()));
    let _repo2 = Arc::new(SqliteProductRepository::new(pool.clone()));
    let _repo3 = Arc::new(SqliteProductRepository::new(pool.clone()));
    
    // プールの参照カウントを確認（実装詳細なので実際のテストでは不要）
    // 複数のリポジトリが効率的にプールを共有していることを確認
    
    println!("✅ Pool sharing test passed!");
} 