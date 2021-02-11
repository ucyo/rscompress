use crate::{Transform, TransformError};

const ALPHABET_SIZE: u8 = 255; // + 1 number of elements

#[derive(Debug)]
pub struct MoveToFront {
    table: Vec<u8>
}

impl MoveToFront {
    fn new() -> Self {
        let table: Vec<u8> = (0u8..=ALPHABET_SIZE).collect();
        MoveToFront { table }
    }
}

impl Default for MoveToFront {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform for MoveToFront {
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            let pos = self.table.iter().position(|p| p == byte);
            let pos = pos.unwrap() + 1; // add 1 since pos is not included
            self.table[..pos].rotate_right(1);
            result.push(pos as u8);
        }
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
        transform::<MoveToFront>(
            "bananaaa".as_bytes(),
            &[99, 99, 111, 2, 2, 2, 1, 1],
        );
    }
}
