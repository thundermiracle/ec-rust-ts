use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::Arc;
use tokio::sync::OnceCell;

pub struct Database {
    pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}

// スレッドセーフなシングルトンインスタンス
static DB_INSTANCE: OnceCell<Arc<Database>> = OnceCell::const_new();

pub async fn init_db(database_url: &str) -> Result<()> {
    let db = Database::new(database_url).await?;
    // すでに初期化されている場合はエラーを返す
    match DB_INSTANCE.set(Arc::new(db)) {
        Ok(_) => Ok(()),
        Err(_) => Err(anyhow::anyhow!("Database already initialized"))
    }
}

pub async fn get_db() -> Result<Arc<Database>> {
    match DB_INSTANCE.get() {
        Some(db) => Ok(db.clone()),
        None => Err(anyhow::anyhow!("Database not initialized"))
    }
}