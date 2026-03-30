use crate::{
    core::signature::{sign, verify},
    model::{address::Address, data::Data, signature::Signature},
    util::key::SK,
};

impl Data {
    #[allow(dead_code)]
    pub fn new(
        node_sk: SK,
        recipient: Address,
        issuer: Address,
        content: String,
    ) -> Result<Self, ()> {
        let Ok(sign) = create_sign_to_data(node_sk, &recipient, &issuer, content.clone()) else {
            return Err(());
        };
        Ok(Self {
            recipient,
            issuer,
            content,
            sign,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.recipient.der, self.issuer.der, self.content
        )
    }

    pub fn to_buf_for_sign(&self) -> Result<Vec<u8>, ()> {
        data_to_buf_for_sign(&self.recipient, &self.issuer, self.content.clone())
    }

    pub fn verify_sign(&self) -> bool {
        self.to_buf_for_sign()
            .map(|buf| verify(&buf, self.recipient.clone(), self.sign.clone()))
            .is_ok()
    }
}
pub fn data_to_buf_for_sign(
    recipient: &Address,
    issuer: &Address,
    content: String,
) -> Result<Vec<u8>, ()> {
    let Ok(recipient_buf) = recipient.key().public_key_to_der() else {
        return Err(());
    };
    let Ok(issuer_buf) = issuer.key().public_key_to_der() else {
        return Err(());
    };
    let content_buf = content.as_bytes().to_vec();
    Ok([recipient_buf, issuer_buf, content_buf].concat())
}

pub fn create_sign_to_data(
    node_sk: SK,
    recipient: &Address,
    issuer: &Address,
    content: String,
) -> Result<Signature, ()> {
    match data_to_buf_for_sign(recipient, issuer, content).and_then(|buf| sign(&buf, node_sk)) {
        Ok(sign) => Ok(sign),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use crate::util::key::generate_pk_and_sk;

    use super::*;

    #[test]
    fn test_create_sign_to_data() {
        let (recipient, node_sk) = generate_pk_and_sk(512).unwrap();
        let (issuer, _) = generate_pk_and_sk(512).unwrap();
        let content = "test".to_string();
        let Ok(signature) =
            create_sign_to_data(node_sk.clone(), &recipient, &issuer, content.clone())
        else {
            panic!();
        };
        assert_eq!(
            sign(
                &data_to_buf_for_sign(&recipient, &issuer, content).unwrap(),
                node_sk
            )
            .unwrap(),
            signature
        );
    }

    #[test]
    fn test_verify_sign() {
        let (recipient, node_sk) = generate_pk_and_sk(512).unwrap();
        let (issuer, _) = generate_pk_and_sk(512).unwrap();
        let content = "test".to_string();
        let data = Data::new(
            node_sk.clone(),
            recipient.clone(),
            issuer.clone(),
            content.clone(),
        )
        .unwrap();
        let Ok(signature) =
            create_sign_to_data(node_sk.clone(), &recipient, &issuer, content.clone())
        else {
            panic!();
        };
        assert!(
            verify(
                &data_to_buf_for_sign(&recipient, &issuer, content).unwrap(),
                recipient,
                signature
            )
            .is_ok()
                && data.verify_sign()
        );
    }
}
