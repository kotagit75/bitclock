use serde::{Deserialize, Serialize};

use crate::{
    model::{proofpool::ProofPool, stamp::Stamp},
    util::key::PK,
};

pub const P2P_PORT: u32 = 62697;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum P2PMessage {
    RequestStamp(PK, usize /*difficulty */),
    ResponseStamp(PK, Stamp),
    UpdateProofpool(ProofPool),
}
