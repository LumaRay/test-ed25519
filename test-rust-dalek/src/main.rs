// https://crates.io/crates/ed25519-dalek
// Generate 2194
// Sign 2277
// Verify 6012

extern crate rand;
extern crate ed25519_dalek;

use rand::rngs::OsRng;
use ed25519_dalek::Keypair;
//use ed25519_dalek::Signature;

use ed25519_dalek::{Signature, Signer};

//use ed25519_dalek::Verifier;

//use ed25519_dalek::{PublicKey, Verifier};

use std::time::{SystemTime};//, UNIX_EPOCH};

//const DATA_SIZE: usize = 512 / 8;
const LOOPS_COUNT: u32 = 100_000;

fn main() {
// First, we need to generate a Keypair, which includes both public and secret halves of an asymmetric key. To do so, we need a cryptographically secure pseudorandom number generator (CSPRNG). For this example, we'll use the operating system's builtin PRNG:
    let mut csprng = OsRng{};

    let mut keypair: Keypair = Keypair::generate(&mut csprng);
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        keypair = Keypair::generate(&mut csprng);
    }
    println!("Generate {:?}", SystemTime::now().duration_since(start).unwrap());

// We can now use this keypair to sign a message:
    let message: &[u8] = b"This is a test of the tsunami a This is a test of the tsunami a ";
    let mut signature: Signature = keypair.sign(message);
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        signature = keypair.sign(message);
    }
    println!("Sign {:?}", SystemTime::now().duration_since(start).unwrap());

// As well as to verify that this is, indeed, a valid signature on that message:
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        assert!(keypair.verify(message, &signature).is_ok());
    }
    println!("Verify {:?}", SystemTime::now().duration_since(start).unwrap());
    //println!("Verify before publishing {:?}", SystemTime::now().duration_since(start).unwrap());

// Anyone else, given the public half of the keypair can also easily verify this signature:
    /*let public_key: PublicKey = keypair.public;
    let start = SystemTime::now();
    for _ in 0..LOOPS_COUNT {
        assert!(public_key.verify(message, &signature).is_ok());
    }
    println!("Verify after receiving {:?}", SystemTime::now().duration_since(start).unwrap());*/
}
