//! # rscompress-transformation
//!
//! This crate implements transformation algorithms to be used during compression.
//!
//! ## Introduction
//! The transformation algorithms implemented in this crate are all reversible.
//! The original order of the data can be reproduced by using the `reverse` operation
//! defined by the `Transform` Trait.
mod runlength;

/// Trait for calculating transformations on byte level
pub trait Transform {
    fn transform(&mut self, source: &[u8]) -> Option<Vec<u8>>;
    fn reverse(&mut self, source: &[u8]) -> Option<Vec<u8>>;
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
