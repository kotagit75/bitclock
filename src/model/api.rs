use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

use crate::util::key::SK;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum APICommand {
    AddPeer(String /*ip */),
    Proof(String /*data */),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ApiOrdering {
    Less,
    Equal,
    Greater,
}
impl From<Ordering> for ApiOrdering {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Less => ApiOrdering::Less,
            Ordering::Equal => ApiOrdering::Equal,
            Ordering::Greater => ApiOrdering::Greater,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SKPair {
    pub sk1: SK,
    pub sk2: SK,
}
