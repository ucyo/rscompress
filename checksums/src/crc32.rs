use super::{Checksum, ChecksumError};
use crc::{crc32, Hasher32};

pub struct CRC32 {
    a: crc32::Digest,
}

impl CRC32 {
    pub fn new() -> Self {
        CRC32 { a: crc32::Digest::new(crc32::IEEE) }
    }
}

impl Checksum for CRC32 {
    fn update(&mut self, data: &[u8]) -> Option<usize> {
        self.a.write(data);
        Some(data.len())
    }
    fn checksum(&self) -> Result<u32, ChecksumError> {
        let c = self.a.sum32();
        Ok(c)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_checksum;

    test_checksum!(test_wikipedia, CRC32, "Wikipedia", 0xadaac02e);
    test_checksum!(test_awesome, CRC32, "Awesome-string-baby", 0x7900b113);
    test_checksum!(test_greatness, CRC32, "This is great", 0xc6314444);
}
