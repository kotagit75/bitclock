use tokio::sync::mpsc::Sender;

use crate::{adapter::p2p::init_p2p, model::event::Event};

pub mod p2p;

pub fn init_adapter(tx: Sender<Event>) {
    tokio::spawn(async move {
        init_p2p(tx).await;
    });
}
