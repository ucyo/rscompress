//! # rscompress-checksums
//!
//! `rscompress_checksums` implements dfferent checksum algorithms for bytes.
//! checksums can be added to the compressed data.
//! This can then be used to check up if the deconstructed data is the same
//! as the original data.
use std::error::Error;
use std::fmt;

mod adler32;
mod crc32;

pub use adler32::Adler32;
pub use crc32::CRC32;

/// Trait for calculating checksums from binary data
pub trait Checksum {
    fn update(&mut self, data: &[u8]) -> Option<usize>;
    fn checksum(&self) -> Result<u32, ChecksumError>;
}

/// An enum representing possible errors during checksum calculation
#[derive(Debug)]
pub enum ChecksumError {
    /// The final checksum can not be calculated
    NoChecksum,
    /// Error during updating of the checksums
    UpdateError,
}

impl Error for ChecksumError {
    fn description(&self) -> &str {
        match *self {
            ChecksumError::NoChecksum => "No checksum",
            ChecksumError::UpdateError => "No update",
        }
    }
}

impl fmt::Display for ChecksumError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChecksumError::NoChecksum => write!(f, "Can not calculate checksum"),
            ChecksumError::UpdateError => write!(f, "Can not update checksum"),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub mod tests {
    //! # Tests
    //! This module defines helper functions for testing checksum algorithms.
    use crate::Checksum;

    /// Helper function for calculating checksum
    pub fn checksum<M: Checksum + Default>(input: &[u8], expected: u32) {
        let mut model: M = Default::default();
        model.update(&input);
        assert_eq!(model.checksum().unwrap(), expected)
    }
}
