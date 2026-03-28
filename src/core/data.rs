use crate::model::data::Data;

impl Data {
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}",
            self.recipient.der, self.issuer.der, self.credential
        )
    }
}
