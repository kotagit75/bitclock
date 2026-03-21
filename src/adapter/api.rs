use axum::{
    Router,
    extract::{self, State},
    response,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::Sender, watch::Receiver};

use crate::model::{address::Address, event::Event, proof::ProofPool};

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
        .route("/query", get(handle_query))
        .route("/query/address", get(handle_query_address))
        .route("/query/pool", get(handle_query_pool))
        .route("/status", get(handle_status))
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

async fn handle_query_address(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
) -> response::Json<Address> {
    response::Json(rx.borrow().address.clone())
}

async fn handle_query_pool(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
) -> response::Json<ProofPool> {
    response::Json(rx.borrow().proof_pool.clone())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum SystemStatusType {
    Running,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct SystemStatus {
    pub status: SystemStatusType,
}
async fn handle_status(
    State(_): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
) -> response::Json<SystemStatus> {
    response::Json(SystemStatus {
        status: SystemStatusType::Running,
    })
}
