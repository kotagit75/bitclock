use openssl::{
    hash::MessageDigest,
    sign::{Signer, Verifier},
};

use crate::{
    model::signature::Signature,
    util::key::{PK, SK},
};

pub fn sign(data: &[u8], sk: SK) -> Result<Signature, ()> {
    let Ok(mut signer) = Signer::new(MessageDigest::sha256(), &sk.key()) else {
        return Err(());
    };
    let Ok(_) = signer.update(data) else {
        return Err(());
    };
    let Ok(sign) = signer.sign_to_vec() else {
        return Err(());
    };
    Ok(sign)
}

pub fn verify(data: &[u8], pk: PK, signature: Signature) -> bool {
    let key = pk.key();
    let Ok(mut verifyer) = Verifier::new(MessageDigest::sha256(), &key) else {
        return false;
    };
    let Ok(_) = verifyer.update(&data) else {
        return false;
    };
    let Ok(result) = verifyer.verify(&signature) else {
        return false;
    };
    result
}
