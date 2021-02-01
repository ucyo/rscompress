
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
