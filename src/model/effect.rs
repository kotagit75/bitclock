use crate::{adapter::p2p::P2PMessage, util::key::PK};

pub enum Effect {
    None,
    CreateStamp(PK, usize /*difficulty */),
    Broadcast(P2PMessage),
}
