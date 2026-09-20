mod config;
mod handlers;
mod routes;

use config::{Config, ConfigLoader};
#[tokio::main]
async fn main() {
    let config = Config::load_env();

    let router = routes::create_router(config.clone());

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
        .await
        .unwrap();

    println!(
        "Server running at http://127.0.0.1:{} with env {:?}",
        config.port,config.environment
    );

    axum::serve(listener, router).await.unwrap();
}
