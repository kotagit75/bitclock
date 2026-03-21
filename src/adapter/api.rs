use axum::Router;
use tokio::sync::mpsc::Sender;

use crate::model::event::Event;

pub const API_PORT: u32 = 3000;
pub async fn init_api(tx: Sender<Event>) {
    let app = Router::new();
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", API_PORT))
        .await
        .unwrap();
    info!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}
