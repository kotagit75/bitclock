use crate::model::client::Client;

impl Client {
    pub fn new(ip_addr: String) -> Self {
        Client { ip_addr }
    }
}
