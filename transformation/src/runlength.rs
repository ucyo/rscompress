//! Run-length Transformation
//!
//! Implementation of a run-length transformation as
//! described [here](https://sites.google.com/site/datacompressionguide/rlt).
use crate::{Transform, TransformError};
use log::info;

const RUN_BYTE_CODE: u8 = 0;

/// Run-length struct to save current byte and metainformation about special cases
#[derive(Debug)]
pub struct RunLength {
    current: Option<u8>,
    reverse_started: bool,
}

impl RunLength {
    pub fn new() -> Self {
        RunLength {
            current: None,
            reverse_started: false,
        }
    }
}

impl Default for RunLength {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation of the Transform trait for Run-Length
impl Transform for RunLength {
    fn transform(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            info!("Transform: {} | {:?}", byte, self);
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
        self.reverse_started = false;
        Ok(result)
    }

    fn reverse(&mut self, source: &[u8]) -> Result<Vec<u8>, TransformError> {
        if source.is_empty() {
            return Err(TransformError::EmptyBufferError);
        }
        let mut result: Vec<u8> = Vec::with_capacity(source.len());
        for byte in source.iter() {
            info!("Reverse: {} | {:?}", byte, self);
            if self.current.is_some() && *byte == RUN_BYTE_CODE && self.reverse_started {
                result.push(self.current.unwrap());
                self.reverse_started = true;
            } else if self.current.is_some()
                && *byte == self.current.unwrap()
                && self.reverse_started
            {
                result.push(RUN_BYTE_CODE);
                self.current = Some(RUN_BYTE_CODE);
                self.reverse_started = true;
            } else {
                result.push(*byte);
                self.current = Some(*byte);
                self.reverse_started = true;
            }
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
        transform::<RunLength>(
            &[8, 2, 2, 2, 24, 32, 32, 1, 24],
            &[
                8,
                2,
                RUN_BYTE_CODE,
                RUN_BYTE_CODE,
                24,
                32,
                RUN_BYTE_CODE,
                1,
                24,
            ],
        );
    }
    #[test]
    fn test_easy_reverses() {
        reverse::<RunLength>(
            &[
                8,
                2,
                RUN_BYTE_CODE,
                RUN_BYTE_CODE,
                24,
                32,
                RUN_BYTE_CODE,
                1,
                24,
            ],
            &[8, 2, 2, 2, 24, 32, 32, 1, 24],
        );
        reverse::<RunLength>(
            &[8, RUN_BYTE_CODE, RUN_BYTE_CODE, 8],
            &[8, 8, 8, RUN_BYTE_CODE],
        );
    }

    #[test]
    fn test_roundtrip() {
        roundtrip::<RunLength>(&[8, 2, 2, 2, 24, 32, 32, 1, 24]);
        roundtrip::<RunLength>(&[8, 8, 8, 8, 2]);
        roundtrip::<RunLength>(&[8, 8, 1, 2, 2]);
        roundtrip::<RunLength>(&[8, 8, 8, 8]);
        roundtrip::<RunLength>(&[RUN_BYTE_CODE, 8, 8, 8]);
        roundtrip::<RunLength>(&[8, 1, 5, 8]);
    }

    #[test]
    fn test_random_roundtrip() {
        random_roundtrip::<RunLength>(100);
        random_roundtrip::<RunLength>(100);
        random_roundtrip::<RunLength>(100);
        random_roundtrip::<RunLength>(100);
        random_roundtrip::<RunLength>(100);
    }
}
