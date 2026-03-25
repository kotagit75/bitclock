use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum APICommand {
    AddPeer(String /*ip */),
    Proof(String /*data */),
}
