use crate::util::key::PK;

pub enum Effect {
    None,
    CreateStamp(PK, usize /*difficulty */),
}
