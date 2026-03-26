use tokio::sync::mpsc::Sender;

use crate::model::{
    api::{APICommand, APIResponse},
    p2p::P2PMessage,
};

#[derive(Debug)]
pub enum Event {
    P2PRequest(P2PMessage),
    APIRequest(APICommand, Sender<APIResponse>),
}
