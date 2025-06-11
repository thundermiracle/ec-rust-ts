use anyhow::Result;
use chrono::Utc;

use crate::frameworks_and_drivers::database::db::get_db;

pub async fn seed_database() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    let now = Utc::now().to_rfc3339();

    // サンプルデータを挿入
    let products = vec![
        ("Laptop", 99999, "High-performance laptop", 10),
        ("Mouse", 2999, "Wireless optical mouse", 50),
        ("Keyboard", 7999, "Mechanical keyboard", 25),
    ];

    for (name, price, description, quantity) in products {
        sqlx::query(
            "INSERT INTO products (name, price, description, quantity, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(name)
        .bind(price)
        .bind(description)
        .bind(quantity)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    println!("Database seeded successfully!");
    Ok(())
} 