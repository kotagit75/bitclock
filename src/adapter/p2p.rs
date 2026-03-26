use axum::{
    Router,
    extract::{self, State},
    routing::post,
};
use tokio::sync::mpsc::Sender;

use crate::model::{
    event::Event,
    p2p::{P2P_PORT, P2PMessage},
};

use crate::model::client::Client;

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

impl Client {
    fn get_url(&self) -> String {
        format!("http://{}:{}", self.ip_addr, P2P_PORT)
    }
    pub async fn write(&self, message: &P2PMessage) {
        let _ = reqwest::Client::new()
            .post(self.get_url())
            .json(message)
            .send()
            .await;
    }
}
