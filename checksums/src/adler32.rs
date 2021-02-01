
use super::Checksum;
struct Adler32 {
    a: u16,
    b: u16,
}

impl Adler32 {
    fn new() -> Self {
        Adler32 {
            a: 1,
            b: 0,
        }
    }
}

impl Checksum for Adler32 {
    fn update(&mut self, data: &[u8]) {
        for byte in data.iter() {
            self.a += *byte as u16 % 65535;
            self.b += self.a % 65535;
            println!("{:?} {:?}", self.a, self.b)
        }
    }
    fn checksum(&self) -> u32 {
        ((self.b as u32) << 16) | self.a as u32
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
        assert_eq!(a.checksum(), 0x11E60398)
    }
}
