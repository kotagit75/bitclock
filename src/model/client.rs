use serde::{Deserialize, Serialize};

use crate::adapter::p2p::{P2P_PORT, P2PMessage};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub ip: String,
}
impl Client {
    pub fn new(ip: String) -> Self {
        Client { ip }
    }
    fn get_url(&self) -> String {
        format!("http://{}:{}", self.ip, P2P_PORT)
    }
    pub async fn write(&self, message: &P2PMessage) {
        let _ = reqwest::Client::new()
            .post(self.get_url())
            .json(message)
            .send()
            .await;
    }
}
