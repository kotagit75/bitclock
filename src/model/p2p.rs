use serde::{Deserialize, Serialize};

use crate::{
    model::{proof::ProofPool, stamp::Stamp},
    util::key::PK,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum P2PMessage {
    RequestStamp(PK, usize /*difficulty */),
    ResponceStamp(PK, Stamp),
    UpdateProofpool(ProofPool),
}
