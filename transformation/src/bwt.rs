use std::todo;
use std::str;
use suffix;
use crate::{Transform, TransformError};
use log::debug;

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
        let mut result = Vec::with_capacity(source.len());
        let temp = str::from_utf8(source).unwrap();
        let s = suffix::SuffixTable::new(temp);
        let table = s.table();
        debug!("Suffixtable: {:?} ({})", table, table.len());
        for ix in table.iter() {
            if *ix as usize == 0 {
                let val = source[source.len() - 1];
                result.push(val);
        } else {
                let val = source[*ix as usize - 1];
                result.push(val);
        }
        }
        self.ix = table.iter().position(|&x| x==0);
        debug!("Suffixtable Index Position: {:?}", self.ix);
        Ok(result)
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
        transform::<BurrowWheeler>("banana".as_bytes(), "nnbaaa".as_bytes());
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
