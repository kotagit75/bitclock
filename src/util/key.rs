use openssl::{
    pkey::{PKey, Private, Public},
    rsa::Rsa,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash)]
pub struct PK {
    pub der: String,
}
impl PartialEq for PK {
    fn eq(&self, other: &Self) -> bool {
        self.der == other.der
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct SK {
    pub der: String,
}
impl PartialEq for SK {
    fn eq(&self, other: &Self) -> bool {
        self.der == other.der
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
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
    pub fn to_pk(&self) -> Result<PK, ()> {
        let Ok(pem) = self.key().public_key_to_pem() else {
            return Err(());
        };
        let Ok(rsa) = Rsa::public_key_from_pem(pem.as_slice()) else {
            return Err(());
        };
        let Ok(key) = PKey::from_rsa(rsa) else {
            return Err(());
        };
        Ok(PK::new(key))
    }
}
