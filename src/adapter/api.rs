use axum::{
    Router,
    extract::{self, State},
    response,
    routing::{get, post},
};
use tokio::sync::{
    mpsc::{self, Sender},
    watch::Receiver,
};

use crate::{
    core::proof::compare_time,
    model::{
        address::Address,
        api::{APICommand, APIResponse, ApiOrdering, SKPair},
        event::Event,
        proof::{Proof, ProofPool},
    },
    util::{
        key::SK,
        status::{SystemStatus, get_status},
    },
};

const API_PORT: u32 = 8080;
pub async fn init_api(tx: Sender<Event>, state_rx: Receiver<crate::model::state::State>) {
    let app = Router::new()
        .route("/", post(handle_command))
        .route("/query", get(handle_query))
        .route("/query/address", get(handle_query_address))
        .route("/query/pool", get(handle_query_pool))
        .route("/query/find", get(handle_query_find_by_sk))
        .route("/query/verify", get(handle_query_verify))
        .route("/query/compare", get(handle_query_compare_time))
        .route("/status", get(handle_status))
        .with_state((tx, state_rx));
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", API_PORT))
        .await
        .unwrap();
    info!("API server is running on http://localhost:{}", API_PORT);
    axum::serve(listener, app).await.unwrap();
}

async fn handle_command(
    State((tx, _)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
    extract::Json(message): extract::Json<APICommand>,
) -> response::Json<APIResponse> {
    let (responce_tx, mut responce_rx): (mpsc::Sender<APIResponse>, mpsc::Receiver<APIResponse>) =
        mpsc::channel(100);
    let _ = tx.send(Event::APIRequest(message, responce_tx)).await;
    response::Json(responce_rx.recv().await.unwrap())
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

async fn handle_query_find_by_sk(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
    extract::Json(sk): extract::Json<SK>,
) -> response::Json<Option<Proof>> {
    response::Json(rx.borrow().proof_pool.clone().find_by_sk(&sk))
}

async fn handle_query_verify(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
    extract::Json(proof): extract::Json<Proof>,
) -> response::Json<bool> {
    response::Json(rx.borrow().proof_pool.clone().verify(&proof))
}

async fn handle_query_compare_time(
    State((_, rx)): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
    extract::Json(sk_pair): extract::Json<SKPair>,
) -> response::Json<Option<ApiOrdering>> {
    let Some(proof1) = rx.borrow().proof_pool.find_by_sk(&sk_pair.sk1) else {
        return response::Json(None);
    };
    let Some(proof2) = rx.borrow().proof_pool.find_by_sk(&sk_pair.sk2) else {
        return response::Json(None);
    };
    let ord = ApiOrdering::from(compare_time(&proof1, &proof2));
    response::Json(Some(ord))
}

async fn handle_status(
    State(_): State<(Sender<Event>, Receiver<crate::model::state::State>)>,
) -> response::Json<SystemStatus> {
    response::Json(get_status())
}
