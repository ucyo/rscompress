//! Move-to-Front Transformation
//!
//! Implementation of the Move-to-front transformation as described
//! [here](https://sites.google.com/site/datacompressionguide/mtf).
//!
use crate::{Transform, TransformError};
use log::debug;

const ALPHABET_SIZE: u8 = 255; // + 1 number of elements

/// Move-to-Front struct to save the table
#[derive(Debug)]
pub struct MoveToFront {
    table: Vec<u8>,
}

impl MoveToFront {
    fn new() -> Self {
        let table: Vec<u8> = (0u8..=ALPHABET_SIZE).collect();
        MoveToFront { table }
    }
    pub fn reset(&mut self) {
        let table: Vec<u8> = (0u8..=ALPHABET_SIZE).collect();
        self.table = table;
    }
    fn rotate(&mut self, pos: usize) {
        self.table[..pos].rotate_right(1);
    }
}

impl Default for MoveToFront {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of the Transformation trait for Move-To-Front
impl Transform for MoveToFront {
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            let pos = self
                .table
                .iter()
                .position(|p| p == byte)
                .ok_or_else(|| TransformError::SymbolNotFound(*byte))?;
            debug!("Found {:?} at {:?}", byte, pos);
            self.rotate(pos + 1);
            result.push(pos as u8);
        }
        Ok(result)
    }
    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        self.reset();
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for pos in source.iter() {
            let ix = *pos as usize;
            debug!(
                "Found element (w/ surround) {:?} [{:?}] {:?}",
                self.table[ix],
                self.table[ix - 1],
                self.table[ix + 1]
            );
            result.push(self.table[ix]);
            self.rotate(ix+1);
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
        transform::<MoveToFront>("bananaaa".as_bytes(), &[98, 98, 110, 1, 1, 1, 0, 0]);
    }

    #[test]
    fn test_easy_reverse() {
        reverse::<MoveToFront>(&[98, 98, 110, 1, 1, 1, 0, 0], "bananaaa".as_bytes());
    }

    #[test]
    fn test_easy_roundtrip() {
        roundtrip::<MoveToFront>("bananaaa".as_bytes());
    }

    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<MoveToFront>(100);
        random_roundtrip::<MoveToFront>(100);
        random_roundtrip::<MoveToFront>(100);
        random_roundtrip::<MoveToFront>(100);
        random_roundtrip::<MoveToFront>(100);
    }
}
