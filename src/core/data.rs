use crate::{
    core::signature::{sign, verify},
    model::{address::Address, data::Data, signature::Signature},
    util::key::SK,
};

impl Data {
    pub fn new(
        node_sk: SK,
        recipient: Address,
        issuer: Address,
        credential: String,
    ) -> Result<Self, ()> {
        let Ok(sign) = create_sign_to_data(node_sk, &recipient, &issuer, credential.clone()) else {
            return Err(());
        };
        Ok(Self {
            recipient,
            issuer,
            credential,
            sign,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.recipient.der, self.issuer.der, self.credential
        )
    }

    pub fn to_buf_for_sign(&self) -> Result<Vec<u8>, ()> {
        data_to_buf_for_sign(&self.recipient, &self.issuer, self.credential.clone())
    }

    pub fn verify_sign(&self) -> bool {
        match self
            .to_buf_for_sign()
            .and_then(|buf| Ok(verify(&buf, self.recipient.clone(), self.sign.clone())))
        {
            Ok(_) => true,
            Err(_) => false,
        }
    }
}
pub fn data_to_buf_for_sign(
    recipient: &Address,
    issuer: &Address,
    credential: String,
) -> Result<Vec<u8>, ()> {
    let Ok(recipient_buf) = recipient.key().public_key_to_der() else {
        return Err(());
    };
    let Ok(issuer_buf) = issuer.key().public_key_to_der() else {
        return Err(());
    };
    let credential_buf = credential.as_bytes().to_vec();
    Ok(vec![recipient_buf, issuer_buf, credential_buf].concat())
}

pub fn create_sign_to_data(
    node_sk: SK,
    recipient: &Address,
    issuer: &Address,
    credential: String,
) -> Result<Signature, ()> {
    match data_to_buf_for_sign(recipient, issuer, credential).and_then(|buf| sign(&buf, node_sk)) {
        Ok(sign) => Ok(sign),
        Err(_) => Err(()),
    }
}
