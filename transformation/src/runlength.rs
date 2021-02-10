use crate::Transform;

const RUN_BYTE_CODE: u8 = 0;

#[derive(Debug)]
struct RunLength {
    current: u8,
    // running: bool,
}

impl RunLength {
    pub fn new() -> Self {
        RunLength { current: 0 }
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
            if self.current == *byte {
                result.push(RUN_BYTE_CODE);
                // self.running = true;
                // running: bool,
            } else if RUN_BYTE_CODE == *byte {
                result.push(self.current);
                self.current = *byte;
                // self.running = false;
            } else {
                result.push(*byte);
                self.current = *byte;
                // self.running = true;
            }
        }
        Some(result)
    }
    fn rtransform(&mut self, source: &[u8]) -> Option<Vec<u8>> {
        todo!()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_transform;

    #[test]
    fn test_rlt_transform() {
        let data = vec![8, 2, 2, 2, 24, 32, 32, 1, 24];
        let expected = vec![8, 2, RUN_BYTE_CODE, RUN_BYTE_CODE, 24, 32, RUN_BYTE_CODE, 1, 24];
        let mut rl = RunLength::new();
        let result = rl.transform(&data).unwrap();

        assert_eq!(result, expected)
    }

    test_transform!(rlt_easy, RunLength,
        vec![8, 2, 2, 2, 24, 32, 32, 1, 24],
        vec![8, 2, RUN_BYTE_CODE, RUN_BYTE_CODE, 24, 32, RUN_BYTE_CODE, 1, 24]);

}
