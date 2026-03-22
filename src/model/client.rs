use serde::{Deserialize, Serialize};

use crate::adapter::p2p::{P2P_PORT, P2PMessage};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub ip_addr: String,
}
impl Client {
    pub fn new(ip_addr: String) -> Self {
        Client { ip_addr }
    }
    fn get_url(&self) -> String {
        format!("http://{}:{}", self.ip_addr, P2P_PORT)
    }
    pub async fn write(&self, message: &P2PMessage) {
        let _ = reqwest::Client::new()
            .post(self.get_url())
            .json(message)
            .send()
            .await;
    }
}

pub const MY_IP_ADDR: &str = "127.0.0.1";
