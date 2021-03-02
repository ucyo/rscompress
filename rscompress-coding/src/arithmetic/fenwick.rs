#![allow(dead_code, unused_variables)]
use crate::arithmetic::Statistics;

pub const NUMBER_SYMBOLS: u16 = 14;

#[derive(Debug)]
struct Fenwick {
    freq: Vec<usize>,
    inc: usize,
}

impl Fenwick {
    pub fn new() -> Self {
        Fenwick {
            // TODO should this be 1 for symbols?
            freq: vec![0, NUMBER_SYMBOLS as usize + 1], // plus 1 for 0
            inc: 1,
        }
    }
    pub fn with_frequencies(frequencies: Vec<usize>) -> Self {
        assert!(frequencies.len() == NUMBER_SYMBOLS as usize + 1); // plus 1 for 0
        let test = frequencies.split_first().unwrap();
        assert!(*test.0 == 0); // first element must be zero
        assert!(test.1.iter().position(|&x| x == 0).is_none()); // all other elements are at least 1
        Fenwick {
            freq: frequencies,
            ..Default::default()
        }
    }
    pub(crate) fn normalize(&mut self) {
        for f in self.freq.iter_mut() {
            *f = (*f >> 1) + (*f == 1) as usize;
        }
    }

    /// This symbol does not represent the original symbol to be encoded
    pub(crate) fn get_h_freq(&self, symbol: u16) -> usize {
        let mut i = symbol as usize;
        let mut result = 0usize;

        while i != 0 {
            result += self.freq[i];
            i = backward(i);
        }
        result
    }
}

fn map(symbol: u8) -> u16 {
    if symbol == 0 {
        NUMBER_SYMBOLS
    } else {
        symbol as u16
    }
}

fn backward(num: usize) -> usize {
    num - (num & (!num + 1))
}

fn forward(num: usize) -> usize {
    num + (num & (!num + 1))
}

impl Default for Fenwick {
    fn default() -> Self {
        Fenwick::new()
    }
}

impl Statistics for Fenwick {
    fn get_total(&self) -> usize {
        self.get_h_freq(NUMBER_SYMBOLS)
    }
    fn get_symbol(&self, target: usize) -> usize {
        todo!()
    }
    fn update_freq_count(&mut self, symbol: u8) {
        todo!()
    }
    fn get_freq_bounds(&self, symbol: u8) -> (usize, usize, usize) {
        let s = map(symbol);
        let lower = self.get_h_freq(s - 1);
        let higher = self.get_h_freq(s);
        let total = self.get_total();
        (lower, higher, total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10]; // sum = 46
        let result: Vec<usize> = vec![0, 1, 1, 1, 3, 1, 4, 1, 10, 3, 5, 2, 8, 1, 5]; // sum = 23
        let mut c = Fenwick::with_frequencies(frequencies);

        c.normalize();
        assert_eq!(c.freq, result);
    }

    #[test]
    fn test_backwards() {
        let f = Fenwick::new();

        assert_eq!(backward(13), 12);
        assert_eq!(backward(8), 0);
        assert_eq!(backward(2), 0);
        assert_eq!(backward(9), 8);
        assert_eq!(backward(2), 0);
    }

    #[test]
    fn test_forward() {
        let f = Fenwick::new();

        assert_eq!(forward(13), 14);
        assert_eq!(forward(14), 16);
        assert_eq!(forward(16), 32);
    }

    #[test]
    fn test_single_frequency_calculation() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        let f = Fenwick::with_frequencies(frequencies);

        // These frequencies are not the lower bounds from the paper, since
        // the implementation considers 0 as a possible value as well
        assert_eq!(f.get_h_freq(1), 1);
        assert_eq!(f.get_h_freq(9), 26);
        assert_eq!(f.get_h_freq(7), 17);
        assert_eq!(f.get_h_freq(8), 20);
        assert_eq!(f.get_h_freq(12), 36);
        assert_eq!(f.get_h_freq(14), 46);
    }

    #[test]
    fn test_internal_mapping() {
        assert_eq!(map(16), 16);
        assert_eq!(map(0), NUMBER_SYMBOLS);
    }

    #[test]
    fn test_frequency_tuple_calculation() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        let f = Fenwick::with_frequencies(frequencies);

        assert_eq!(f.get_freq_bounds(8), (17, 20, 46));
        assert_eq!(f.get_freq_bounds(3), (2, 3, 46));
        assert_eq!(f.get_freq_bounds(10), (26, 31, 46));
        assert_eq!(f.get_freq_bounds(14), (37, 46, 46));
    }

    // TODO Test if increment works

    // TODO Test if increment with 256 elements work
}
