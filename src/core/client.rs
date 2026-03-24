use crate::{
    adapter::p2p::{P2P_PORT, P2PMessage},
    model::client::Client,
};

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
