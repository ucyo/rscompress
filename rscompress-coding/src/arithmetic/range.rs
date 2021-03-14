use std::{fmt, fmt::Display};

type INTERVAL = u32;

#[derive(Debug)]
struct RangeEnc {
    low: INTERVAL,
    rng: INTERVAL,
}

impl Display for RangeEnc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RangeEnc(\n  low: {:032b},\n  rng: {:032b},\n)", self.low, self.rng)
    }
}

impl RangeEnc {
    pub fn new() -> Self {
        RangeEnc { low: 0, rng: !0 }
    }
}

impl Default for RangeEnc {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printing() {
        let enc = RangeEnc::new();
        println!("{}", enc);

        assert_eq!(enc.low, 0);
        assert_eq!(enc.rng, INTERVAL::MAX);
    }
}
