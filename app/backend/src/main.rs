use axum::{Router, http::HeaderValue, middleware, response::Response};
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

pub use error::{Error, Result};

mod application;
mod domain;
mod error;
mod infrastructure;
mod presentation;

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
    infrastructure::database::db::init_db(database_url).await?;

    // 依存関係の解決
    let container = Arc::new(
        infrastructure::get_container()
            .await
            .map_err(|e| anyhow::anyhow!("Failed to initialize container: {}", e))?,
    );

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => {
            // CORS設定を作成
            let cors = CorsLayer::new()
                .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
                .allow_methods(Any)
                .allow_headers(Any);

            let app = Router::new()
                .merge(presentation::routes())
                .layer(cors) // CORSレイヤーを追加
                .layer(middleware::map_response(main_response_mapper))
                .with_state(container); // アプリケーション状態としてコンテナを追加

            let addr = "127.0.0.1:4000";
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

            println!("->> Listening on {addr}");
            axum::serve(listener, app).await.unwrap();
        }
        Commands::Migration => {
            println!("Running migrations...");
            infrastructure::database::migrations::run_migrations(database_url).await?;
            println!("Migrations completed successfully!");
        }
        Commands::Seed => {
            println!("Seeding database...");
            infrastructure::database::run_seeds().await?;
            infrastructure::database::seed_sample_products().await?;
            println!("Database seeded successfully!");
        }
        Commands::Reset => {
            println!("Resetting database...");
            infrastructure::database::clear::clear_database().await?;
            infrastructure::database::run_seeds().await?;
            infrastructure::database::seed_sample_products().await?;
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
