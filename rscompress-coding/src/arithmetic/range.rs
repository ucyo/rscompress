use std::{fmt, fmt::Display};

type INTERVAL = u32;

#[derive(Debug)]
struct RangeEnc {
    low: INTERVAL,
    rng: INTERVAL,
}

impl Display for RangeEnc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RangeEnc(\n  low: {:032b},\n  rng: {:032b},\n)",
            self.low, self.rng
        )
    }
}

impl RangeEnc {

    /// Create new Range Encoder
    pub fn new() -> Self {
        RangeEnc { low: 0, rng: !0 }
    }

    /// Calculate new `low` and `rng` values
    pub fn next_interval(&self, low: u32, high: u32, total: u32) -> (INTERVAL, INTERVAL) {
        let range = self.rng / total;
        let new_low = self.low + low * range;

        let new_rng = if low == high {
            self.rng - new_low
        } else {
            (high - low) * range
        };
        (new_low, new_rng)
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
    fn test_init() {
        let enc = RangeEnc::new();
        assert_eq!(enc.low, 0);
        assert_eq!(enc.rng, INTERVAL::MAX);
    }
    #[test]
    fn test_update() {
        let enc = RangeEnc::new();
        let (l, r) = enc.next_interval(5, 10, 10);
        assert_eq!(l, (INTERVAL::MAX >> 1) - 2);
        assert_eq!(r, (INTERVAL::MAX >> 1));
    }
}
