#![allow(dead_code, unused_variables)]
use crate::arithmetic::Statistics;

/// The currently allowed number of symbols
pub const NUMBER_SYMBOLS: u16 = 14;

/// Fenwick's Tree Structure for implicit O(log n) frequency counts
///
/// Implicit tree structure with O(log n) for updating and retrieving cumulative count for frequencies.
/// TODO: Currently the model is limited to a known number of symbols. This must be updated to allow arbitary number of symbols.
#[derive(Debug)]
pub struct Fenwick {
    freq: Vec<usize>,
    inc: usize,
}

impl Fenwick {
    /// Generate a new Fenwick tree with NUMBER_SYMBOLS options
    pub fn new() -> Self {
        Fenwick {
            // `+1` is necessary since `self.freq[0]` must always be set to `0`
            freq: vec![0; NUMBER_SYMBOLS as usize + 1],
            inc: 1,
        }
    }

    /// Generate a new Fenwick tree with predefined frequencies
    pub fn with_frequencies(frequencies: Vec<usize>) -> Self {
        // Check if the predefined frequencies cover the whole symbol range
        assert!(frequencies.len() == NUMBER_SYMBOLS as usize + 1);

        // The first element of the frequency count must be 0 and
        let (first, _) = frequencies.split_first().unwrap();
        assert!(*first == 0);

        // TODO: Add check if frequencies table has all the properties of a Fenwick Tree
        Fenwick {
            freq: frequencies,
            ..Default::default()
        }
    }

    /// Normalize frequency counts if the total_count of symbols is close to `usize::MAX`
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

    /// Get reference to inner frequency counts
    pub fn get_ref(&self) -> &Vec<usize> {
        self.freq.as_ref()
    }
}

/// Mapping of symbols to support arbitary i.e. `u16::MAX` number of entries
fn map(symbol: u8) -> u16 {
    if symbol == 0 {
        NUMBER_SYMBOLS
    } else {
        symbol as u16
    }
}

/// Backwards calculation of entries needed for retrieving cum. freq. counts in O(log n)
fn backward(num: usize) -> usize {
    num - (num & (!num + 1))
}

/// Backwards calculation of entries needed for adding cum. freq. counts in O(log n)
fn forward(num: usize) -> usize {
    num + (num & (!num + 1))
}

/// Implementation of defaults
impl Default for Fenwick {
    fn default() -> Self {
        Fenwick::new()
    }
}

/// Implementation of the Statistics trait
impl Statistics for Fenwick {
    fn get_total(&self) -> usize {
        self.get_h_freq(NUMBER_SYMBOLS)
    }
    fn get_symbol(&self, target: usize) -> usize {
        let mut s = 0usize;
        let mut t = target;
        let mut mid = 2usize.pow((NUMBER_SYMBOLS as f32).log2().floor() as u32);
        while mid > 0 {
            let nmid = s + mid;
            if nmid <= NUMBER_SYMBOLS as usize && self.freq[nmid] <= t {
                t -= self.freq[nmid];
                s = nmid;
            }
            mid /= 2;
        }
        s
    }
    fn update_freq_count(&mut self, symbol: u8) {
        let mut ix = map(symbol) as usize;
        while ix <= self.freq.len() {
            self.freq[ix] += self.inc;
            ix = forward(ix);
        }
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
        let result2: Vec<usize> = vec![0, 1, 1, 1, 1, 1, 2, 1, 5, 1, 2, 1, 4, 1, 2]; // sum = 11
        let mut c = Fenwick::with_frequencies(frequencies);

        c.normalize();
        assert_eq!(c.freq, result);
        c.normalize();
        assert_eq!(c.freq, result2);
    }

    #[test]
    fn test_total_count() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10]; // sum = 46
        let mut c = Fenwick::with_frequencies(frequencies);

        assert_eq!(c.get_total(), 46);
        c.normalize();
        assert_eq!(c.get_total(), 23);
        c.normalize();
        assert_eq!(c.get_total(), 11);
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

    #[test]
    fn test_frequency_increment() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        let mut f = Fenwick::with_frequencies(frequencies);

        assert_eq!(f.get_h_freq(7), 17);
        f.update_freq_count(7);
        f.update_freq_count(7);
        assert_eq!(f.get_h_freq(7), 19);
        f.update_freq_count(3);
        f.update_freq_count(8);
        f.update_freq_count(12);
        assert_eq!(f.get_h_freq(3), 4);
        assert_eq!(f.get_h_freq(8), 24);
        assert_eq!(f.get_h_freq(12), 41);
    }

    #[test]
    fn test_symbol_recovery() {
        let frequencies: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        let f = Fenwick::with_frequencies(frequencies);
        assert_eq!(f.get_symbol(28), 9);
        assert_eq!(f.get_symbol(5), 3);
        assert_eq!(f.get_symbol(13), 5);
        assert_eq!(f.get_symbol(36), 12);
        assert_eq!(f.get_symbol(40), 13);
        assert_eq!(f.get_symbol(41), 13);
        assert_eq!(f.get_symbol(17), 7);
        assert_eq!(f.get_symbol(18), 7);
        assert_eq!(f.get_symbol(19), 7);
    }

    #[test]
    fn test_build_example_table() {
        let mut f = Fenwick::new();
        let expected: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        let counts: Vec<usize> = vec![0, 1, 1, 1, 4, 3, 5, 2, 3, 6, 5, 4, 1, 1, 9];
        for (sym, count) in counts.iter().enumerate() {
            for _ in 0..*count {
                f.update_freq_count(sym as u8);
            }
        }
        assert_eq!(f.freq.as_ref(), expected);
    }

    // TODO Test if increment with 256 elements work
}
