use axum::{
    Router,
    middleware,
    response::Response,
};
use clap::{Parser, Subcommand};
use std::sync::Arc;

pub use error::{Error, Result};

mod error;
mod interface_adapters;
mod application;
mod domain;
mod frameworks_and_drivers;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the server
    Serve,
    /// Run migrations
    Migration,
    /// Seed the database
    Seed,
    /// Reset the database
    Reset,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let database_url = "sqlite:data/db.sqlite";
    frameworks_and_drivers::database::db::init_db(database_url).await?;
    
    // 依存関係の解決
    let container = Arc::new(frameworks_and_drivers::get_container());
    
    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => {
            let app = Router::new()
                .merge(interface_adapters::products::routes())
                .layer(middleware::map_response(main_response_mapper))
                .with_state(container);  // アプリケーション状態としてコンテナを追加

            let addr = "127.0.0.1:4000";
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

            println!("->> Listening on {addr}");
            axum::serve(listener, app).await.unwrap();
        },
        Commands::Migration => {
            println!("Running migrations...");
            frameworks_and_drivers::database::migrations::run_migrations(database_url).await?;
            println!("Migrations completed successfully!");
        },
        Commands::Seed => {
            println!("Seeding database...");
            frameworks_and_drivers::database::run_seeds().await?;
            frameworks_and_drivers::database::seed_sample_products().await?;
            println!("Database seeded successfully!");
        },
        Commands::Reset => {
            println!("Resetting database...");
            frameworks_and_drivers::database::clear::clear_database().await?;
            frameworks_and_drivers::database::run_seeds().await?;
            frameworks_and_drivers::database::seed_sample_products().await?;
            println!("Database reset successfully!");
        }
    }
    
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> main response mapper");
    println!();

    res
}
