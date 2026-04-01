use vdf::{PietrzakVDFParams, VDF, VDFParams};

const VDF_DIFFICULTY: u64 = 68;
const VDF_BITS: u16 = 512;

pub fn verify_solution(challenge: &[u8], solution: &[u8]) -> bool {
    let vdf = PietrzakVDFParams(VDF_BITS).new();
    vdf.verify(challenge, VDF_DIFFICULTY, solution).is_ok()
}

pub fn solve(challenge: &[u8]) -> Result<Vec<u8>, vdf::InvalidIterations> {
    let vdf = PietrzakVDFParams(VDF_BITS).new();
    vdf.solve(challenge, VDF_DIFFICULTY)
}
