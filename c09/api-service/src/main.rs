use router::create_router;

mod api;
mod error;
mod router;
mod todo;

fn init_tracing() {
    use tracing_subscriber::{
        filter::LevelFilter, fmt, prelude::*, EnvFilter,
    };

    let rust_log = std::env::var(EnvFilter::DEFAULT_ENV)
        .unwrap_or_else(|_| "sqlx=info,tower_http=debug,info".to_string());

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .parse_lossy(rust_log),
        )
        .init();
}

async fn init_dbpool() -> Result<sqlx::Pool<sqlx::Sqlite>, sqlx::Error> {
    use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
    use std::str::FromStr;

    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:db.sqlite".to_string());

    let dbpool = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::from_str(&db_connection_str)?
                .create_if_missing(true),
        )
        .await
        .expect("can't connect to database");

    sqlx::migrate!()
        .run(&dbpool)
        .await
        .expect("database migration failed");

    Ok(dbpool)
}

#[tokio::main]
async fn main() {
    init_tracing();

    let dbpool = init_dbpool().await.expect("couldn't initialize DB pool");

    let router = create_router(dbpool).await;

    let bind_addr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .expect("couldn't bind to address");
    axum::serve(listener, router).await.expect("server failed");
}
