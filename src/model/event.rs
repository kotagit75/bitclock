use crate::adapter::{api::APIRequest, p2p::P2PMessage};

#[derive(Debug)]
pub enum Event {
    P2PRequest(P2PMessage),
    APIRequest(APIRequest),
}
