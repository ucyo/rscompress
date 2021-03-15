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
        let (mut low, mut rng) = self.next_interval(low, high, total);
        let mut output = 0usize;
        // Normalization of variables `low` and `rng`
        loop {
            if rng >= RANGE_THRESHOLD {
                break;
            } else if low >= MASK {
                // carry bit
                unimplemented!()
            }
            out[output] = ((low & MASK) >> EXCESS_BITS_IN_INTERVAL) as u8;
            rng = (rng << 8) + 0xFF;
            low <<= 8;
            output += 1;
        }
        // Assigning new low and rng to Coder
        self.low = low;
        self.rng = rng;

        if output != 0 {
            Some(output)
        } else {
            None
        }
    }

    fn finish(&mut self, out: &mut [u8]) -> Option<usize> {
        let mut output = 0usize;
        if self.low >= MASK {
            // check carry bits
            unimplemented!()
        }
        // It is actually not necessary to put out all bytes.
        // Any code between low and low+range is possible
        out[output] = ((self.low >> 24) & 0xFF) as u8;
        output += 1;
        out[output] = ((self.low >> 16) & 0xFF) as u8;
        output += 1;
        out[output] = ((self.low >> 8) & 0xFF) as u8;
        output += 1;
        out[output] = (self.low & 0xFF) as u8;
        output += 1;

        if output != 0 {
            Some(output)
        } else {
            None
        }
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
    use crate::arithmetic::fenwick::fenwick_with_string_frequencies as ffreq;
    use crate::arithmetic::FenwickStatistics;
    use crate::arithmetic::Statistics;

    fn get_skewed_example_two() -> FenwickStatistics<String> {
        let symbols: Vec<String> = vec![
            "A3".to_string(),
            "A2".to_string(),
            "A1".to_string(),
            "eof".to_string(),
        ];
        let freq: Vec<usize> = vec![23162, 975000, 1837, 1];
        ffreq(freq, symbols)
    }

    fn get_skewed_example_one() -> FenwickStatistics<String> {
        let symbols: Vec<String> = vec!["A3".to_string(), "A2".to_string(), "A1".to_string()];
        let freq: Vec<usize> = vec![231, 9750, 18];
        ffreq(freq, symbols)
    }

    fn get_swiss_example() -> FenwickStatistics<String> {
        let symbols: Vec<String> = vec![
            "S".to_string(),
            "W".to_string(),
            "I".to_string(),
            "M".to_string(),
            "_".to_string(),
        ];
        let freq: Vec<usize> = vec![5, 1, 2, 1, 1];
        ffreq(freq, symbols)
    }

    #[test]
    fn test_init() {
        let enc = RangeCoder::new();
        assert_eq!(enc.low, 0);
        assert_eq!(enc.rng, INTERVAL::MAX);
    }
    #[test]
    fn test_next_interval() {
        let enc = RangeCoder::new();
        let ff = get_swiss_example();
        let (l, h, t) = ff.get_freq_bounds(&"S".to_string());
        println!("{} {} {}", l, h, t);
        let (l, r) = enc.next_interval(l as u32, h as u32, t as u32);

        assert_eq!(l, 0);
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

    #[test]
    fn test_edge_case_one() {
        // a3 a3 a3 a3 a3
        assert!(false)
    }

    #[test]
    fn test_edge_case_two() {
        // a3 a3 a3 a3 eof
        assert!(false)
    }

    #[test]
    fn test_edge_case_three() {
        // a2 a2 a1 a3 a3
        assert!(false)
    }
}
