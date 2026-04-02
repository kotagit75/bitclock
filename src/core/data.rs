use openssl::error::ErrorStack;

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
    ) -> Result<Self, ErrorStack> {
        create_sign_to_data(node_sk, &recipient, &issuer, content.clone()).map(|sign| Self {
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

    pub fn to_buf_for_sign(&self) -> Vec<u8> {
        data_to_buf_for_sign(&self.recipient, &self.issuer, self.content.clone())
    }

    pub fn verify_sign(&self) -> bool {
        verify(
            &self.to_buf_for_sign(),
            self.recipient.clone(),
            self.sign.clone(),
        )
    }
}
pub fn data_to_buf_for_sign(recipient: &Address, issuer: &Address, content: String) -> Vec<u8> {
    format!("{:?} {:?} {:?}", recipient.der, issuer.der, content)
        .as_bytes()
        .to_vec()
}

pub fn create_sign_to_data(
    node_sk: SK,
    recipient: &Address,
    issuer: &Address,
    content: String,
) -> Result<Signature, ErrorStack> {
    sign(&data_to_buf_for_sign(recipient, issuer, content), node_sk)
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
            sign(&data_to_buf_for_sign(&recipient, &issuer, content), node_sk).unwrap(),
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
                &data_to_buf_for_sign(&recipient, &issuer, content),
                recipient,
                signature
            ) && data.verify_sign()
        );
    }
}
