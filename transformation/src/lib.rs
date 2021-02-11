mod runlength;

pub trait Transform {
    fn transform(&mut self, source: &[u8]) -> Option<Vec<u8>>;
    fn reverse(&mut self, source: &[u8]) -> Option<Vec<u8>>;
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::Transform;
    use rand::{RngCore, rngs::OsRng};

    pub fn transform<M: Transform+Default>(input: &[u8], expected: &[u8]) {
        let mut model: M = Default::default();
        let result = model.transform(&input).unwrap();
        assert_eq!(result, expected)
    }

    pub fn reverse<M: Transform+Default>(input: &[u8], expected: &[u8]) {
        let mut model: M = Default::default();
        let result = model.reverse(&input).unwrap();
        assert_eq!(result, expected)
    }

    pub fn roundtrip<M: Transform+Default>(input: &[u8]) {
        let mut model: M = Default::default();
        let tmp = model.transform(&input).unwrap();
        let result = model.reverse(&tmp).unwrap();
        assert_eq!(result, input)
    }

    pub fn random_roundtrip<M: Transform+Default>(trips: usize) {
        for _ in 0..trips {
            let mut input = [0u8; 25];
            OsRng.fill_bytes(&mut input);
            let mut model: M = Default::default();
            let tmp = model.transform(&input).unwrap();
            let result = model.reverse(&tmp).unwrap();
            print!("Input: {:?}", input);
            assert_eq!(result, input)
        }
    }

}
