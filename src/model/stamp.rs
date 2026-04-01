use serde::{Deserialize, Serialize};

use crate::{
    model::{address::Address, signature::Signature},
    util::key::PK,
};

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Stamp {
    pub address: Address,
    pub count: u32,
    pub pk: PK,
    pub nonce: u32,
    pub solution: Vec<u8>,
    pub id: usize,
    pub sign: Signature,
}
