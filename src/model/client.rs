use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Client {
    pub ip_addr: String,
}

pub const MY_IP_ADDR: &str = "127.0.0.1";
