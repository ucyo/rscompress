//! CRC32 checksum
//!
//! Implementation of the CRC32 checksum algorithm as described [here](https://en.wikipedia.org/wiki/Cyclic_redundancy_check).
use super::{Checksum, ChecksumError};
use crc::{crc32, Hasher32};
use log::{debug, info};

/// CRC32 struct to save inner Digest element from `crc32` crate
pub struct CRC32 {
    a: crc32::Digest,
}

impl CRC32 {
    /// Generate new CRC32 struct
    pub fn new() -> Self {
        info!("New CRC32 checksum created");
        CRC32 {
            a: crc32::Digest::new(crc32::IEEE),
        }
    }
}

/// Use the new function for generating the default implementation
impl Default for CRC32 {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of the Checksum trait for CRC32
impl Checksum for CRC32 {
    fn update(&mut self, data: &[u8]) -> Option<usize> {
        debug!("Update checksum using bytes of length {}", data.len());
        self.a.write(data);
        Some(data.len())
    }
    fn checksum(&self) -> Result<u32, ChecksumError> {
        let c = self.a.sum32();
        debug!("Checksum is {}", c);
        Ok(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::checksum;

    #[test]
    fn test_words() {
        checksum::<CRC32>("Wikipedia".as_bytes(), 0xadaac02e);
        checksum::<CRC32>("Awesome-string-baby".as_bytes(), 0x7900b113);
        checksum::<CRC32>("This is great".as_bytes(), 0xc6314444);
    }
}
