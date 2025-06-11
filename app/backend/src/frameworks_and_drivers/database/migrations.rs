use anyhow::Result;
use sqlx::Sqlite;

use crate::frameworks_and_drivers::database::db::get_db;

pub async fn run_migrations(database_url: &str) -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();
    
    // テーブル作成（簡単な例）
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            price INTEGER NOT NULL,
            description TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    println!("Migrations completed successfully!");
    Ok(())
} 