//! # rscompress-checksums
//!
//! `rscompress_checksums` implements dfferent checksum algorithms for bytes.
//! checksums can be added to the compressed data.
//! This can then be used to check up if the deconstructed data is the same
//! as the original data.

mod adler32;
use std::error::Error;
use std::fmt;

/// Trait for calculating checksums from binary data
trait Checksum {
    fn update(&mut self, data: &[u8]) -> Option<usize>;
    fn checksum(&self) -> Result<u32, ChecksumError>;
}

/// An enum representing possible errors during checksum calculation
#[derive(Debug)]
enum ChecksumError {
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

#[macro_export]
macro_rules! test_checksum {
    ($func_name:ident, $method:ident, $test_string:expr, $expected:expr) => {
        #[test]
        fn $func_name() {
            let mut a = $method::new();
            let data = $test_string.as_bytes();
            a.update(&data);
            assert_eq!(a.checksum().unwrap(), $expected)
        }
    };
}
