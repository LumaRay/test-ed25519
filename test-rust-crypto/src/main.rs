// https://crates.io/crates/rust-crypto
// Generate 3824
// Sign 3971
// Verify 12805

use crypto::ed25519::{keypair, signature, verify};

use std::time::{SystemTime};//, UNIX_EPOCH};

const LOOPS_COUNT: u32 = 100_000;

fn main() {
// Generate a keypair
    let seed: &[u8] = b"This is a seed..This is a seed..";
    let (mut secret_key, mut public_key) = keypair(seed);
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        (secret_key, public_key) = keypair(seed);
    }
    println!("Generate {:?}", SystemTime::now().duration_since(start).unwrap());

// Sign the data
    let message: &[u8] = b"This is a test of the tsunami a This is a test of the tsunami a ";
    let mut sig = signature(message, &secret_key);
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        sig = signature(message, &secret_key);
    }
    println!("Sign {:?}", SystemTime::now().duration_since(start).unwrap());

// Verify the data
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        assert!(verify(message, &public_key, &sig));
    }
    println!("Verify {:?}", SystemTime::now().duration_since(start).unwrap());
}
