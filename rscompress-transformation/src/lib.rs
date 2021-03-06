//! This crate implements transformation algorithms to be used during compression.
//!
//! # Introduction
//! The transformation algorithms implemented in this crate are all reversible.
//! The original order of the data can be reproduced by using the `reverse` operation
//! defined by the `Transform` Trait.
use std::fmt;
use std::{error::Error, fmt::Display};
mod bwt;
mod movetofront;
mod runlength;

pub use bwt::BurrowWheeler;
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
    /// Missing index of Burrow Wheeler
    MissingIndex,
    /// Missing symbol in the reverse mapping of Burrow Wheeler
    MissingMapping(u8),
    /// Missing count of symbol
    MissingCountMap(u8, usize),
}

impl Error for TransformError {
    fn description(&self) -> &str {
        match *self {
            TransformError::EmptyBufferError => "Empty Buffer",
            TransformError::SymbolNotFound(_val) => "No Symbol",
            TransformError::MissingIndex => "Missing index position",
            TransformError::MissingMapping(_val) => "No Mapping",
            TransformError::MissingCountMap(_, _) => "Can not find enough occurences of symbol",
        }
    }
}

impl Display for TransformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TransformError::EmptyBufferError => write!(f, "Can not read because buffer is empty"),
            TransformError::SymbolNotFound(val) => write!(f, "Symbol [{:?}] not found", val),
            TransformError::MissingIndex => write!(f, "There is no index given"),
            TransformError::MissingMapping(val) => write!(f, "Mapping for [{:?}] is missing", val),
            TransformError::MissingCountMap(sym, c) => {
                write!(f, "Missing {:?}. occurence of symbol '{:?}'", c + 1, sym)
            }
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
        println!("Input:       {:?}", input);
        let mut model: M = Default::default();
        let tmp = model.transform(&input).unwrap();
        println!("Transformed: {:?}", tmp);
        let result = model.reverse(&tmp).unwrap();
        println!("Reversed:    {:?}", result);
        assert_eq!(result, input)
    }

    /// Helper function for testing random transformation roundtrips
    pub fn random_roundtrip<M: Transform + Default>(trips: usize, size: usize) {
        for _ in 0..trips {
            let mut input = vec![0u8; size];
            OsRng.fill_bytes(&mut input);
            let mut model: M = Default::default();
            let tmp = model.transform(&input).unwrap();
            let result = model.reverse(&tmp).unwrap();
            if result != input {
                println!("Input:       {:?}", input);
                println!("Transformed: {:?}", tmp);
                println!("Reversed:    {:?}", result);
            }
            assert_eq!(result, input)
        }
    }
}
