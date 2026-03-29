use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SystemStatusType {
    Running,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemStatus {
    pub status: SystemStatusType,
    pub memory_usage_bytes: Option<u64>,
}

pub fn get_status() -> SystemStatus {
    let mut sys = System::new_all();
    sys.refresh_all();
    let pid = Pid::from_u32(std::process::id());
    SystemStatus {
        status: SystemStatusType::Running,
        memory_usage_bytes: sys.process(pid).map(|process| process.memory()),
    }
}
