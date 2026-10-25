#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::var("LEARNINGOS_CATALOG")
        .unwrap_or_else(|_| "learningos-v2-catalog.sqlite3".into());
    let assets = std::env::var("LEARNINGOS_ASSETS").unwrap_or_else(|_| "apps/web/public".into());
    let address = std::env::var("LEARNINGOS_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".into());
    let catalog = learningos_server::Catalog::open(path)?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    eprintln!("LearningOS listening on http://{address}");
    axum::serve(listener, learningos_server::router(catalog, assets))
        .with_graceful_shutdown(async {
            let _ = tokio::signal::ctrl_c().await;
        })
        .await?;
    Ok(())
}
