mod adler32;
use std::error::Error;
use std::fmt;

trait Checksum {
    fn update(&mut self, data: &[u8]);
    fn checksum(&self) -> u32;
}

#[derive(Debug)]
enum ChecksumError {
    NoChecksum,
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
