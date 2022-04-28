use std::net::SocketAddr;

pub mod di;
pub mod layer;
pub mod router;

pub async fn run() {
    let app = router::route();
    let app = di::provide(app).await;
    let app = layer::layer(app);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
