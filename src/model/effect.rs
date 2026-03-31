use tokio::sync::mpsc::Sender;

use crate::{
    model::{api::APIResponse, p2p::P2PMessage},
    util::key::PK,
};

#[derive(Debug, Clone)]
pub enum Effect {
    CreateStamp(PK, usize /*difficulty */, usize /*id */),
    Broadcast(P2PMessage),
    APIResponse(Sender<APIResponse>, APIResponse),
}
