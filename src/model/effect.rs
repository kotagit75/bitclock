use tokio::sync::mpsc::Sender;

use crate::{
    model::{api::APIResponse, p2p::P2PMessage},
    util::key::PK,
};

#[derive(Debug)]
pub enum Effect {
    CreateStamp(PK, usize /*difficulty */),
    Broadcast(P2PMessage),
    APIResponce(Sender<APIResponse>, APIResponse),
}
