use anyhow::Result;

use crate::frameworks_and_drivers::database::db::get_db;

pub async fn clear_database() -> Result<()> {
    let db = get_db().await?;
    let pool = db.get_pool();

    // products テーブルを全削除
    sqlx::query("DELETE FROM products")
        .execute(pool)
        .await?;

    println!("Database cleared successfully!");
    Ok(())
}
