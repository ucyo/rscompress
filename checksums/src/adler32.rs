//! Adler32 checksum
//!
//! Implementation of the Adler32 checksum algorithm as described [here](https://en.wikipedia.org/wiki/Adler-32).
use super::{Checksum, ChecksumError};
use log::{debug, info};

/// Adler32 struct to save normal and aggregated sum
#[derive(Debug)]
struct Adler32 {
    a: u16,
    b: u16,
}

impl Adler32 {

    /// Generate new Adler32 struct
    fn new() -> Self {
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
        return Some(data.len());
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
    #[test]
    fn it_works() {
        let mut a = Adler32::new();
        let data: Vec<u8> = vec![87, 105, 107, 105, 112, 101, 100, 105, 97];

        a.update(&data);
        assert_eq!(a.checksum().unwrap(), 0x11E60398)
    }
}
