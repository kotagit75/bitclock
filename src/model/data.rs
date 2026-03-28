use serde::{Deserialize, Serialize};

use crate::model::address::Address;

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Data {
    pub recipient: Address,
    pub issuer: Address,
    pub credential: String,
}
