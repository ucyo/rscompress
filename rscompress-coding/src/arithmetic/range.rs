#![allow(dead_code)]
use std::{fmt, fmt::Display, io::Write, io::Read};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use super::Statistics;

type INTERVAL = u32;
const INTERVAL_BITS: u32 = 32;
const OUTPUT_BITS: u32 = 8;
const EXCESS_BITS_IN_INTERVAL: u32 = INTERVAL_BITS - OUTPUT_BITS;
const RANGE_THRESHOLD: u32 = 1 << EXCESS_BITS_IN_INTERVAL;
const MASK: u32 = 0xFF << EXCESS_BITS_IN_INTERVAL;

#[derive(Debug)]
pub struct RangeCoder {
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

impl Default for RangeCoder {
    fn default() -> Self {
        Self::new()
    }
}

impl RangeCoder {
    /// Create new Range Encoder
    pub fn new() -> Self {
        RangeCoder { low: 0, rng: !0 }
    }

    /// Reset values
    pub fn reset(&mut self) {
        self.low = 0; self.rng = !0;
    }

    /// Calculate new `low` and `rng` values
    fn next_interval(&self, low: u32, high: u32, total: u32) -> (INTERVAL, INTERVAL) {
        let range = self.rng / total;
        let new_low = self.low + low * range;

        let new_rng = if low == high {
            self.rng - new_low
        } else {
            (high - low) * range
        };
        (new_low, new_rng)
    }

    /// Drink the symbols
    fn update(&mut self, low: u32, high: u32, total: u32, out: &mut [u8]) -> usize {
        let (mut low, mut rng) = self.next_interval(low, high, total);

        let mut output = 0usize;
        loop {
            // Normalization of variables `low` and `rng`
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
        self.low = low;
        self.rng = rng;

        output
    }

    /// Finish drinking the symbols
    fn finish(&mut self, mut out: &mut [u8]) -> usize {
        let mut output = 0usize;
        if self.low >= MASK {
            // check carry bits and counter
            unimplemented!()
        }
        // It is actually not necessary to put out all bytes.
        // Any code between low and low+range is possible
        out.write_u32::<BigEndian>(self.low).unwrap();
        output += 4;

        output
    }

    /// Get truncated index position of symbol
    fn get_index(&self, code: u32, total: u32) -> u32 {
        assert!(self.low <= code && code <= (self.low + self.rng));
        ((code - self.low) * total) / self.rng
    }

}

pub struct Encoder<W: Write> {
    inner: W,
    coder: RangeCoder,
}

impl<W: Write> Encoder<W> {
    pub fn new(w: W) -> Self {
        Encoder { inner: w, coder: RangeCoder::default() }
    }
    pub fn drink<Sym, Stat: Statistics<Symbol=Sym>>(&mut self, symbol: &Sym, ctx: &mut Stat) -> Result<usize, std::io::Error> {
        let (low, high, total) = ctx.get_freq_bounds(symbol);
        ctx.update_freq_count(symbol).unwrap();
        let mut out = [0u8; 4];
        let count = self.coder.update(low as u32, high as u32, total as u32, &mut out);
        self.inner.write(&out[..count])
    }
    pub fn finish(&mut self) -> Result<usize, std::io::Error> {
        let mut out = [0u8; 4];
        self.coder.finish(&mut out);
        self.inner.write(&out[..4])
    }
    pub fn get_ref(&self) -> &W {
        &self.inner
    }
    pub fn get_mut(&mut self) -> &mut W {
        &mut self.inner
    }
    pub fn into_inner(self) -> W {
        self.inner
    }
    pub fn get_ref_coder(&self) -> &RangeCoder {
        &self.coder
    }
    pub fn get_mut_coder(&mut self) -> &mut RangeCoder {
        &mut self.coder
    }
    pub fn get_coder(self) -> RangeCoder {
        self.coder
    }
}


pub struct Decoder<R: Read> {
    inner: R,
    coder: RangeCoder,
    value: u32,
    bytecount: usize,
}

impl<R: Read> Decoder<R> {
    pub fn new(mut r: R, bytes: usize) -> Self {
        let value: u32 = r.read_u32::<BigEndian>().unwrap();
        Decoder { inner: r, coder: RangeCoder::new(), bytecount: bytes, value}
    }
    fn fill(&mut self, mut count: usize) {
        while count > 0 {
            let val = self.inner.read_u8().unwrap();
            self.value = (self.value << 8) + val as u32;
            count -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arithmetic::fenwick::fenwick_with_string_frequencies as ffreq;
    use crate::arithmetic::FenwickStatistics;
    use std::io::Cursor;

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
            "_".to_string(),
            "M".to_string(),
            "I".to_string(),
            "W".to_string(),
            "S".to_string(),
        ];
        let freq: Vec<usize> = vec![1, 1, 2, 1, 5];
        ffreq(freq, symbols)
    }

    #[test]
    fn test_init() {
        let writer = Cursor::new(Vec::new());
        let enc = Encoder::new(writer);

        assert_eq!(enc.get_ref_coder().low, 0);
        assert_eq!(enc.get_ref_coder().rng, INTERVAL::MAX);
    }

    #[test]
    fn test_single_coding() {
        let writer = Cursor::new(Vec::new());
        let mut enc = Encoder::new(writer);
        let mut ff = get_swiss_example();
        enc.drink(&"S".to_string(), &mut ff).unwrap();

        assert_eq!(enc.get_ref_coder().low, (INTERVAL::MAX >> 1) - 2);
        assert_eq!(enc.get_ref_coder().rng, (INTERVAL::MAX >> 1) - 2);
    }

    #[test]
    fn test_coding() {
        let writer = Cursor::new(Vec::new());
        let mut enc = Encoder::new(writer);
        let mut ff = get_swiss_example();
        let testword: Vec<String> = vec!["S", "W", "I", "S", "S", "_", "M", "I", "S", "S"]
            .iter_mut()
            .map(|x| x.to_string())
            .collect();
        for b in testword {
            enc.drink(&b, &mut ff).unwrap();
        }
    }

    #[test]
    fn test_edge_case_one() {
        // TODO a3 a3 a3 a3 a3
        assert!(false)
    }

    #[test]
    fn test_edge_case_two() {
        // TODO a3 a3 a3 a3 eof
        assert!(false)
    }

    #[test]
    fn test_edge_case_three() {
        // TODO a2 a2 a1 a3 a3
        assert!(false)
    }
}
