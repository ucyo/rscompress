//! Adler32 checksum
//!
//! Implementation of the Adler32 checksum algorithm as described [here](https://en.wikipedia.org/wiki/Adler-32).
use super::{Checksum, ChecksumError};
use log::{debug, info};

/// Adler32 struct to save normal and aggregated sum
#[derive(Debug, Default)]
pub struct Adler32 {
    a: u16,
    b: u16,
}

impl Adler32 {
    /// Generate new Adler32 struct
    pub fn new() -> Self {
        info!("New Adler32 checksum");
        Adler32 { a: 1, b: 0 }
    }
}

/// Implementation of the Checksum trait for Adler32
impl Checksum for Adler32 {
    fn update(&mut self, data: &[u8]) -> Option<usize> {
        for byte in data.iter() {
            self.a += *byte as u16 % u16::MAX;
            self.b += self.a % u16::MAX;
            debug!("Adler32 Update: {}, New State: {:?}", byte, self)
        }
        Some(data.len())
    }
    fn checksum(&self) -> Result<u32, ChecksumError> {
        let result = ((self.b as u32) << 16) | self.a as u32;
        info!("Adler32 Checksum: {}", result);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_checksum;

    test_checksum!(test_wikipedia, Adler32, "Wikipedia", 0x11E60398);
    test_checksum!(test_awesome, Adler32, "Awesome-string-baby", 0x49D50761);
    test_checksum!(test_greatness, Adler32, "This is great", 0x20AF04C8);
}
