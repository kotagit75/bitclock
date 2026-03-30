use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::model::proof::Proof;

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ProofPool {
    pub pool: HashSet<Proof>,
}
