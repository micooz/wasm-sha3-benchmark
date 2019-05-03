use wasm_bindgen::prelude::*;
use sha3::{Digest, Sha3_256};

#[wasm_bindgen]
pub fn sha3_256(text: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(text);
    format!("{:x}", hasher.result())
}

#[test]
fn test_sha3_256() {
    assert_eq!(sha3_256("abc"), "3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532");
}
