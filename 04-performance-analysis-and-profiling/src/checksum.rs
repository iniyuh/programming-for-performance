use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Default)]
#[derive(Clone)]
pub struct Checksum(String);

impl Checksum {
    // Initialize the checksum with the SHA256 hash of the input string
    pub fn with_sha256(sha: &str) -> Self {
        let digest = Sha256::digest(sha.as_bytes());
        Self(hex::encode(digest.as_slice()))
    }

    // XOR the two checksums
    pub fn update(&mut self, rhs: Self) {
        if self.0.is_empty() {
            *self = rhs;
        } else if !rhs.0.is_empty() {
            let mut a = hex::decode(&self.0).unwrap();
            let mut b = hex::decode(&rhs.0).unwrap();
            assert_eq!(a.len(), b.len());

            // In place XOR
            for (i, val) in a.iter_mut().enumerate() {
                *val = *val ^ b[i];
            }

            *self = Checksum(hex::encode(a))
        };
    }
}

impl fmt::Display for Checksum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
