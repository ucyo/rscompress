use crate::Transform;

const RUN_BYTE_CODE: u8 = 0;

#[derive(Debug)]
struct RunLength {
    current: Option<u8>,
}

impl RunLength {
    pub fn new() -> Self {
        RunLength { current: None }
    }
}

impl Default for RunLength {
    fn default() -> Self {
        Self::new()
    }
}


impl Transform for RunLength {
    fn transform(&mut self, source: &[u8]) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            println!("Transform: {} | {:?}", byte, self);
            if self.current.is_some() && self.current.unwrap() == *byte {
                result.push(RUN_BYTE_CODE);
            } else if self.current.is_some() && RUN_BYTE_CODE == *byte {
                result.push(self.current.unwrap());
                self.current = Some(*byte);
            } else {
                result.push(*byte);
                self.current = Some(*byte);
            }
        }
        Some(result)
    }

    // TODO: There is a problem if the last value in the data is the same as the first one.
    fn reverse(&mut self, source: &[u8]) -> Option<Vec<u8>> {
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            println!("Reverse: {} | {:?}", byte, self);
            if self.current.is_some() && *byte == RUN_BYTE_CODE {
                result.push(self.current.unwrap());
            } else if self.current.is_some() && *byte == self.current.unwrap() {
                // This case must recognise that it is not the first one called in reverse case
                result.push(RUN_BYTE_CODE);
                self.current = Some(RUN_BYTE_CODE);
            } else {
                result.push(*byte);
                self.current = Some(*byte);
            }
        }
        Some(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_transform, test_reverse, test_roundtrip};

    test_transform!(rlt_easy, RunLength,
        vec![8, 2, 2, 2, 24, 32, 32, 1, 24],
        vec![8, 2, RUN_BYTE_CODE, RUN_BYTE_CODE, 24, 32, RUN_BYTE_CODE, 1, 24]);

    test_reverse!(rlt_easy_reverse, RunLength,
        vec![8, 2, RUN_BYTE_CODE, RUN_BYTE_CODE, 24, 32, RUN_BYTE_CODE, 1, 24],
        vec![8, 2, 2, 2, 24, 32, 32, 1, 24]);

    test_reverse!(rlt_reverse, RunLength,
        vec![8, RUN_BYTE_CODE, RUN_BYTE_CODE, 8],
        vec![8, 8, 8, 0]);

    test_roundtrip!(rlt_easy_round, RunLength, vec![8, 2, 2, 2, 24, 32, 32, 1, 24]);
    test_roundtrip!(rlt_round_ending, RunLength, vec![8, 8, 8, 8, 2]);
    test_roundtrip!(rlt_round_ending2, RunLength, vec![8, 8, 1, 2, 2]);
    test_roundtrip!(rlt_round_ending3, RunLength, vec![8, 8, 8, 8]);
    test_roundtrip!(rlt_round_ending4, RunLength, vec![8, 1, 5, 8]);

}
