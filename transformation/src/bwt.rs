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
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError)
        }
        debug!("{:?}", source);
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
        let mut sorted = source.to_vec();
        sorted.sort_unstable();
        let mut counts = vec![0usize; sorted.len()];

        let mut counter = 0;
        let mut last_letter = sorted.first().unwrap();
        let mut ix: usize = 1;
        for sym in sorted[1..].iter() {
            if sym == last_letter {
                counter += 1;
            } else {
                counter = 0;
                last_letter = sym;
            }
            counts[ix] = counter;
            ix += 1;
        }

        println!("Source: {:?}", source);
        println!("Sorted: {:?}", sorted);
        println!("Counts: {:?}", counts);
        println!("Self: {:?}", self);
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        let tmp = str::from_utf8(source).unwrap();
        let suf = suffix::SuffixTable::new(tmp);
        for _ in 0..source.len() {
            let reversed = sorted[self.ix.unwrap()];
            let c = counts[self.ix.unwrap()];
            let ff = [reversed;1];
            let utf_reversed = str::from_utf8(&ff).unwrap();
            println!("Search {:?}th letter of {:?} ({:?})", c, reversed, utf_reversed);
            result.push(reversed);
            let mut pos = suf.positions(utf_reversed).to_vec();
            pos.sort_unstable();
            println!("Positions {:?}", pos);
            self.ix = Some(pos[c] as usize);
            println!("New ix: {:?}", self.ix);
        }
        Ok(result)

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
