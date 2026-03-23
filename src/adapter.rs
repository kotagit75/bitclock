use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    watch::{self},
};

use crate::{
    adapter::{api::init_api, p2p::init_p2p},
    model::{event::Event, state::State},
};

pub mod api;
pub mod p2p;

pub fn init_adapter(state: State, api_port: u32) -> (Receiver<Event>, watch::Sender<State>) {
    let (tx, event_rx): (Sender<Event>, Receiver<Event>) = mpsc::channel(100);
    let (state_tx, state_rx): (watch::Sender<State>, watch::Receiver<State>) =
        watch::channel(state);

    let tx_clone = tx.clone();
    let tx_clone2 = tx.clone();
    let rx_clone = state_rx.clone();
    tokio::spawn(async move {
        init_p2p(tx_clone).await;
    });
    tokio::spawn(async move {
        init_api(tx_clone2, rx_clone, api_port).await;
    });
    return (event_rx, state_tx);
}
