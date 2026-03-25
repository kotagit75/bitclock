use crate::{model::p2p::P2PMessage, util::key::PK};

#[derive(Debug)]
pub enum Effect {
    None,
    CreateStamp(PK, usize /*difficulty */),
    Broadcast(P2PMessage),
}
