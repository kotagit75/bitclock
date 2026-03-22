use axum::{
    Router,
    extract::{self, State},
    routing::post,
};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

use crate::{
    model::{event::Event, proof::ProofPool, stamp::Stamp},
    util::key::PK,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum P2PMessage {
    RequestStamp(PK, usize /*difficulty */),
    ResponceStamp(PK, Stamp),
    UpdateProofpool(ProofPool),
}

pub const P2P_PORT: u32 = 3000;
pub async fn init_p2p(tx: Sender<Event>) {
    let app = Router::new()
        .route("/", post(handle_message))
        .with_state(tx);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", P2P_PORT))
        .await
        .unwrap();
    info!("P2P server is running on http://localhost:{}", P2P_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_message(
    State(tx): State<Sender<Event>>,
    extract::Json(message): extract::Json<P2PMessage>,
) -> &'static str {
    let _ = tx.send(Event::P2PRequest(message)).await;
    ""
}
