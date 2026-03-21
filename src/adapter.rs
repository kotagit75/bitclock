use tokio::sync::{mpsc::Sender, watch::Receiver};

use crate::{
    adapter::{api::init_api, p2p::init_p2p},
    model::{event::Event, state::State},
};

pub mod api;
pub mod p2p;

pub fn init_adapter(tx: Sender<Event>, state_rx: Receiver<State>) {
    let tx_clone = tx.clone();
    let tx_clone2 = tx.clone();
    let rx_clone = state_rx.clone();
    tokio::spawn(async move {
        init_p2p(tx_clone).await;
    });
    tokio::spawn(async move {
        init_api(tx_clone2, rx_clone).await;
    });
}
