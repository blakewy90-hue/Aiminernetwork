use sha2::{Sha256, Digest};
use ed25519_dalek::{Keypair, Signer};

pub fn hash(output: &str) -> Vec<u8> {
    Sha256::digest(output.as_bytes()).to_vec()
    }

    pub fn sign(hash: &[u8], keypair: &Keypair) -> Vec<u8> {
        keypair.sign(hash).to_bytes().to_vec()
        }