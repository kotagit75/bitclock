use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemStatusType {
    Running,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStatus {
    pub status: SystemStatusType,
}

pub fn get_status() -> SystemStatus {
    SystemStatus {
        status: SystemStatusType::Running,
    }
}
