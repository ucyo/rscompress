#![allow(dead_code)]
use std::{fmt, fmt::Display};

const OUTPUT_BITS: u32 = 8;

type INTERVAL = u32;
const INTERVAL_BITS: u32 = 32;

const EXCESS_BITS_IN_INTERVAL: u32 = INTERVAL_BITS - OUTPUT_BITS;
const RANGE_THRESHOLD: u32 = 1 << EXCESS_BITS_IN_INTERVAL;
const MASK: u32 = 0xFF << EXCESS_BITS_IN_INTERVAL;

#[derive(Debug)]
struct RangeCoder {
    low: INTERVAL,
    rng: INTERVAL,
}

impl Display for RangeCoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RangeEnc(\n  low: {:032b},\n  rng: {:032b},\n)",
            self.low, self.rng
        )
    }
}

impl RangeCoder {
    /// Create new Range Encoder
    pub fn new() -> Self {
        RangeCoder { low: 0, rng: !0 }
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

    pub fn code(&mut self, low: u32, high: u32, total: u32, out: &mut [u8]) -> Option<usize> {
        let result = None::<usize>;
        let (low, rng) = self.next_interval(low, high, total);

        // Normalization loop
        loop {
            if rng >= RANGE_THRESHOLD {
                // enough room in range
                break;
            } else {
                unimplemented!()
            }
        }

        // Assigning new low and rng to Coder
        self.low = low;
        self.rng = rng;
        result
    }
}

impl Default for RangeCoder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let enc = RangeCoder::new();
        assert_eq!(enc.low, 0);
        assert_eq!(enc.rng, INTERVAL::MAX);
    }
    #[test]
    fn test_update() {
        let enc = RangeCoder::new();
        let (l, r) = enc.next_interval(5, 10, 10);

        assert_eq!(l, (INTERVAL::MAX >> 1) - 2);
        assert_eq!(r, (INTERVAL::MAX >> 1) - 2);
    }

    #[test]
    fn test_coding() {
        let mut enc = RangeCoder::new();
        let mut output = vec![0];
        enc.code(5, 10, 10, &mut output);

        assert_eq!(enc.low, (INTERVAL::MAX >> 1) - 2);
        assert_eq!(enc.rng, (INTERVAL::MAX >> 1) - 2);
    }
}
