use openssl::{
    error::ErrorStack,
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PK {
    pub der: String,
}
impl PK {
    pub fn new(pk: PKey<Public>) -> Self {
        PK {
            der: hex::encode(pk.public_key_to_der().unwrap()),
        }
    }
    pub fn key(&self) -> PKey<Public> {
        PKey::public_key_from_der(&hex::decode(self.der.clone()).unwrap()).unwrap()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct SK {
    pub der: String,
}
impl SK {
    pub fn new(sk: PKey<Private>) -> Self {
        SK {
            der: hex::encode(sk.private_key_to_der().unwrap()),
        }
    }
    pub fn key(&self) -> PKey<Private> {
        PKey::private_key_from_der(&hex::decode(self.der.clone()).unwrap()).unwrap()
    }
    pub fn to_pk(&self) -> Result<PK, ErrorStack> {
        self.key()
            .public_key_to_pem()
            .and_then(|pem| Rsa::public_key_from_pem(pem.as_slice()))
            .and_then(PKey::from_rsa)
            .map(PK::new)
    }
}

pub fn generate_pk_and_sk(bits: u32) -> Result<(PK, SK), ErrorStack> {
    Rsa::generate(bits)
        .and_then(PKey::from_rsa)
        .map(SK::new)
        .and_then(|sk| sk.clone().to_pk().map(|pk| (pk, sk)))
}
