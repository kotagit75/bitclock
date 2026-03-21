use axum::{
    Router,
    extract::{self, State},
    response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::Sender, watch::Receiver};

use crate::model::event::Event;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum APIRequest {
    None,
    AddPeer(String /*ip */),
    Proof(String /*data */),
}

pub const API_PORT: u32 = 8080;
pub async fn init_api(tx: Sender<Event>, state_rx: Receiver<crate::model::state::State>) {
    let app = Router::new()
        .route("/", post(handle_request))
        .route("/q", get(handle_query))
        .with_state((tx, state_rx));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", API_PORT))
        .await
        .unwrap();
    info!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    State((tx, _)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
    extract::Json(message): extract::Json<APIRequest>,
) -> &'static str {
    let _ = tx.send(Event::APIRequest(message)).await;
    ""
}

async fn handle_query(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
) -> response::Json<crate::model::state::State> {
    response::Json(rx.borrow().clone())
}
