use tokio::sync::mpsc::Sender;

use crate::{
    adapter::{api::init_api, p2p::init_p2p},
    model::event::Event,
};

pub mod api;
pub mod p2p;

pub fn init_adapter(tx: Sender<Event>) {
    let tx_clone = tx.clone();
    let tx_clone2 = tx.clone();
    tokio::spawn(async move {
        init_p2p(tx_clone).await;
    });
    tokio::spawn(async move {
        init_api(tx_clone2).await;
    });
}
