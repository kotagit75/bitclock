use axum::{
    Router,
    extract::{self, State},
    routing::post,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::model::event::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum APIRequest {
    None,
    AddPeer(String /*ip */),
    Proof(String /*data */),
}

pub const API_PORT: u32 = 8080;
pub async fn init_api(tx: Sender<Event>) {
    let app = Router::new()
        .route("/", post(handle_request))
        .with_state(tx);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", API_PORT))
        .await
        .unwrap();
    info!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    State(tx): State<Sender<Event>>,
    extract::Json(message): extract::Json<APIRequest>,
) -> &'static str {
    let _ = tx.send(Event::APIRequest(message));
    ""
}
