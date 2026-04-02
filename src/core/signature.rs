use openssl::{
    error::ErrorStack,
    hash::MessageDigest,
    sign::{Signer, Verifier},
};

use crate::{
    model::signature::Signature,
    util::key::{PK, SK},
};

pub fn sign(data: &[u8], sk: SK) -> Result<Signature, ErrorStack> {
    Signer::new(MessageDigest::sha256(), &sk.key()).and_then(|mut signer| {
        match signer.update(data) {
            Ok(_) => signer.sign_to_vec(),
            Err(e) => Err(e),
        }
    })
}

pub fn verify(data: &[u8], pk: PK, signature: Signature) -> bool {
    Verifier::new(MessageDigest::sha256(), &pk.key())
        .and_then(|mut verifyer| match verifyer.update(data) {
            Ok(_) => verifyer.verify(&signature).or_else(|e| Err(e)),
            Err(e) => Err(e),
        })
        .is_ok()
}
