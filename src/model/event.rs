use crate::adapter::p2p::P2PMessage;

pub enum Event {
    None,
    P2PRequest(P2PMessage),
}
