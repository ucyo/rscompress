mod arithmetic;

pub trait Code {
    fn encode(&mut self, source: &[u8]) -> Result<Vec<u8>, CodeError>;
    fn decode(&mut self, source: &[u8]) -> Result<Vec<u8>, CodeError>;
}

#[derive(Debug)]
pub enum CodeError {}
