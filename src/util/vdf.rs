use vdf::{PietrzakVDFParams, VDF, VDFParams};

//
const VDF_DIFFICULTY: u64 = 68;

pub fn verify_solution(challenge: &[u8], solution: &[u8]) -> bool {
    let vdf = PietrzakVDFParams(1024).new();
    let result = vdf.verify(challenge, VDF_DIFFICULTY, solution).is_ok();
    println!("{:?}", result);
    result
}

pub fn solve(challenge: &[u8]) -> Result<Vec<u8>, vdf::InvalidIterations> {
    let vdf = PietrzakVDFParams(1024).new();
    vdf.solve(challenge, VDF_DIFFICULTY)
}
