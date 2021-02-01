mod adler32;

trait Checksum {
    fn update(&mut self, data: &[u8]);
    fn checksum(&self) -> u32;
}
