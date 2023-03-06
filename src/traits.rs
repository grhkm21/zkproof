#![allow(unused_imports)]
use curve25519_dalek::scalar::Scalar;

// Commitment scheme consists of two functions:
// Commit and Verify
pub trait Commitment<T> {
    // Input: message
    // Output: (commitment string, opening string)
    fn commit(message: Scalar) -> (T, Scalar);

    // Input: (message, commitment string, opening string)
    // Output: Validity of commitment
    fn verify(message: Scalar, commit: T, opening: Scalar) -> bool;
}

pub trait InteractiveProver {}
