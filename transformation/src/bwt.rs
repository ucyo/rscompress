use std::todo;

use suffix;
use crate::{Transform, TransformError};

#[derive(Debug)]
pub struct BurrowWheeler {
    ix: Option<usize>,
}

impl BurrowWheeler {
    pub fn new() -> Self {
        BurrowWheeler { ix: None }
    }
    pub fn reset(&mut self) {
        self.ix = None
    }
}

impl Default for BurrowWheeler {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform for BurrowWheeler {
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        todo!()
    }
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::{random_roundtrip, reverse, roundtrip, transform};

    #[test]
    fn test_easy_transforms() {
        transform::<BurrowWheeler>("compressioncode".as_bytes(), "neodrsooccimpse".as_bytes());
    }

    #[test]
    fn test_easy_reverse() {
        reverse::<BurrowWheeler>("neodrsooccimpse".as_bytes(), "compressioncode".as_bytes());
    }

    #[test]
    fn test_easy_roundtrip() {
        roundtrip::<BurrowWheeler>("compressioncode".as_bytes());
    }
    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
        random_roundtrip::<BurrowWheeler>(100);
    }
}
