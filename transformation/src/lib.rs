//! # rscompress-transformation
//!
//! This crate implements transformation algorithms to be used during compression.
//!
//! ## Introduction
//! The transformation algorithms implemented in this crate are all reversible.
//! The original order of the data can be reproduced by using the `reverse` operation
//! defined by the `Transform` Trait.
use std::fmt;
use std::{error::Error, fmt::Display};
mod movetofront;
mod runlength;

pub use movetofront::MoveToFront;
pub use runlength::RunLength;

/// Trait for calculating transformations on byte level
pub trait Transform {
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError>;
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError>;
}

/// An enum representing possible errors during transformation
#[derive(Debug)]
pub enum TransformError {
    /// Buffer is empty
    EmptyBufferError,
    /// Symbol is not found
    SymbolNotFound(u8),
}

impl Error for TransformError {
    fn description(&self) -> &str {
        match *self {
            TransformError::EmptyBufferError => "Buffer is empty",
            TransformError::SymbolNotFound(val) => "Symbol not found",
        }
    }
}

impl Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransformError::EmptyBufferError => write!(f, "Can not read because buffer is empty"),
            TransformError::SymbolNotFound(val) => write!(f, "Symbol [{:?}] not found", val),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub mod tests {
    //! # Tests
    //! This module defines helper functions for testing transformation algorithms.
    use crate::Transform;
    use rand::{rngs::OsRng, RngCore};

    /// Helper function for testing one-way transformation
    pub fn transform<M: Transform + Default>(input: &[u8], expected: &[u8]) {
        let mut model: M = Default::default();
        let result = model.transform(&input).unwrap();
        assert_eq!(result, expected)
    }

    /// Helper function for testing reverse transformation
    pub fn reverse<M: Transform + Default>(input: &[u8], expected: &[u8]) {
        let mut model: M = Default::default();
        let result = model.reverse(&input).unwrap();
        assert_eq!(result, expected)
    }

    /// Helper function for testing transformation roundtrips
    pub fn roundtrip<M: Transform + Default>(input: &[u8]) {
        let mut model: M = Default::default();
        let tmp = model.transform(&input).unwrap();
        let result = model.reverse(&tmp).unwrap();
        assert_eq!(result, input)
    }

    /// Helper function for testing random transformation roundtrips
    pub fn random_roundtrip<M: Transform + Default>(trips: usize) {
        for _ in 0..trips {
            let mut input = [0u8; 10_000];
            OsRng.fill_bytes(&mut input);
            let mut model: M = Default::default();
            let tmp = model.transform(&input).unwrap();
            let result = model.reverse(&tmp).unwrap();
            if result != input {
                print!("Input: {:?}", input);
            }
            assert_eq!(result, input)
        }
    }
}
