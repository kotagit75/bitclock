use crate::model::client::Client;

impl Client {
    pub fn new(ip_addr: String) -> Self {
        Client { ip_addr }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_new() {
        let client = Client::new("127.0.0.1".to_string());
        assert_eq!(client.ip_addr, "127.0.0.1");
    }
}
