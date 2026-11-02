use axum::Router;

pub async fn serve(
    app: Router,
    host: &str,
    port: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let address = format!("{host}:{port}");
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Server berjalan di http://{address}");
    axum::serve(listener, app).await?;
    Ok(())
}
