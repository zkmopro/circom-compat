//! Arkworks - Circom Compatibility layer
//!
//! Provides bindings to Circom's R1CS, for Groth16 Proof and Witness generation in Rust.

pub mod circom;
pub use circom::{CircomCircuit, CircomReduction};

#[cfg(feature = "ethereum")]
pub mod ethereum;

mod zkey;
pub use zkey::{read_zkey, FieldSerialization, ZkeyHeaderReader};
