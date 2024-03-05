//! Protobuf fuzzing & testing harness for Solana programs.

#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

pub mod fixture;
pub mod program_runtime;
