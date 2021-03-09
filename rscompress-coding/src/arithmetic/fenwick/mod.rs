#![allow(dead_code, unused_variables)]
use crate::arithmetic::Statistics;
use crate::arithmetic::StatisticsError;
use log::debug;
use map::Map;
use std::collections::HashSet;

pub(crate) mod map;

/// Fenwick's Tree Structure for implicit O(log n) frequency counts
///
/// Implicit tree structure with O(log n) for updating and retrieving cumulative count for frequencies.
#[derive(Debug)]
pub struct Fenwick<M> {
    freq: Vec<usize>,
    inc: usize,
    map: M,
}

impl<M: Map> Fenwick<M> {
    /// Generate a new Fenwick tree with NUMBER_SYMBOLS options
    pub fn new() -> Self {
        Fenwick {
            // `+1` is necessary since `self.freq[0]` must always be set to `0`
            freq: vec![0; 1],
            inc: 1,
            map: M::default(),
        }
    }

    /// Normalize frequency counts if the total_count of symbols is close to `usize::MAX`
    pub(crate) fn normalize(&mut self) {
        for f in self.freq.iter_mut() {
            *f = (*f >> 1) + (*f == 1) as usize;
        }
    }

    /// Get H count for symbol at index
    pub(crate) fn get_h_freq(&self, ix: usize) -> usize {
        let mut ix = ix;
        let mut result = 0usize;

        while ix != 0 {
            result += self.freq[ix];
            ix = backward(ix);
        }
        result
    }

    /// Get reference to inner frequency counts
    pub fn get_ref(&self) -> &Vec<usize> {
        self.freq.as_ref()
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
impl<M: Map> Default for Fenwick<M> {
    fn default() -> Self {
        Fenwick::new()
    }
}

/// Implementation of the Statistics trait
impl<M: Map> Statistics for Fenwick<M> {
    type Symbol = M::Input;

    fn get_total(&self) -> usize {
        self.get_h_freq(self.map.alphabet_size())
    }

    fn get_symbol(&self, target: usize) -> Result<&Self::Symbol, StatisticsError> {
        let mut ix = 0usize;
        let mut t = target;
        let mut mid = 2usize.pow((self.map.alphabet_size() as f32).log2().floor() as u32);
        while mid > 0 {
            let nmid = ix + mid;
            if nmid <= self.map.alphabet_size() as usize && self.freq[nmid] <= t {
                t -= self.freq[nmid];
                ix = nmid;
            }
            mid /= 2;
        }
        Ok(self.map.get_symbol_at(ix)?)
    }

    fn update_freq_count(&mut self, symbol: &Self::Symbol) -> Result<(), StatisticsError> {
        debug!("Update freq of [{:?}] at {:?}", symbol, self);
        let ix = self.map.get_index_of(symbol).ok();
        match ix {
            // Symbol has been seen before
            Some(mut ix) => {
                while ix < self.freq.len() {
                    self.freq[ix] += self.inc;
                    ix = forward(ix);
                }
            }
            // New Symbol
            None => {
                assert_eq!(self.map.alphabet_size() + 1, self.freq.len());
                let n = self.map.install(symbol);
                // Add the correct log freq counts
                // See paper page 267, section 3.2, paragraph 2
                if n % 2 == 1 {
                    self.freq.push(1);
                } else {
                    let mut pow = 2u32;
                    loop {
                        if (n % 2usize.pow(pow)) == 2usize.pow(pow - 1) {
                            let mut sum = 1usize;
                            for j in 0..pow - 1 {
                                sum += self.freq[n - 2usize.pow(j)]
                            }
                            self.freq.push(sum);
                            break;
                        } else {
                            pow += 1;
                        }
                    }
                }
            }
        };
        Ok(())
    }

    fn get_freq_bounds(&self, symbol: &Self::Symbol) -> (usize, usize, usize) {
        let ix = self.map.get_index_of(symbol).unwrap();
        let lower = self.get_h_freq(ix - 1);
        let higher = self.get_h_freq(ix);
        let total = self.get_total();
        (lower, higher, total)
    }

    fn feed(&mut self, data: &[Self::Symbol]) -> Result<(), StatisticsError> {
        for symbol in data {
            self.update_freq_count(symbol)?;
        }
        Ok(())
    }
}

// TODO: Integrate methods into struct [low priority]

pub fn fenwick_with_binary_frequencies(
    frequencies: Vec<usize>,
    symbols: Vec<u8>,
) -> Fenwick<map::Cartographer<u8>> {
    assert_eq!(frequencies.len(), symbols.len());
    assert!(frequencies.iter().fold(true, |acc, &s| acc & (s != 0usize)));
    assert_eq!(symbols.len(), symbols.iter().collect::<HashSet<_>>().len());
    let mut f = Fenwick::<map::Cartographer<u8>>::new();
    for (x, mut count) in symbols.iter().zip(frequencies) {
        while count > 0 {
            f.update_freq_count(x).unwrap();
            count -= 1;
        }
    }
    f
}

pub fn fenwick_with_string_frequencies(
    frequencies: Vec<usize>,
    symbols: Vec<String>,
) -> Fenwick<map::Cartographer<String>> {
    assert_eq!(frequencies.len(), symbols.len());
    assert!(frequencies.iter().fold(true, |acc, &s| acc & (s != 0usize)));
    assert_eq!(symbols.len(), symbols.iter().collect::<HashSet<_>>().len());
    let mut f = Fenwick::<map::Cartographer<String>>::new();
    for (x, mut count) in symbols.iter().zip(frequencies) {
        while count > 0 {
            f.update_freq_count(x).unwrap();
            count -= 1;
        }
    }
    f
}

pub fn fenwick_with_vector_frequencies(
    frequencies: Vec<usize>,
    symbols: Vec<Vec<u8>>,
) -> Fenwick<map::Cartographer<Vec<u8>>> {
    assert_eq!(frequencies.len(), symbols.len());
    assert!(frequencies.iter().fold(true, |acc, &s| acc & (s != 0usize)));
    assert_eq!(symbols.len(), symbols.iter().collect::<HashSet<_>>().len());
    let mut f = Fenwick::<map::Cartographer<Vec<u8>>>::new();
    for (x, mut count) in symbols.iter().zip(frequencies) {
        while count > 0 {
            f.update_freq_count(x).unwrap();
            count -= 1;
        }
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;
    use map::Cartographer;

    fn get_example_from_paper() -> Fenwick<Cartographer<u8>> {
        let sym: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let freq: Vec<usize> = vec![1, 1, 1, 4, 3, 5, 2, 3, 6, 5, 4, 1, 1, 9];
        fenwick_with_binary_frequencies(freq, sym)
    }

    #[test]
    fn test_binary_fenwick_init() {
        let f = Fenwick::<Cartographer<u8>>::new();

        assert_eq!(f.get_ref().len(), 1);
        assert_eq!(f.map.get_ref().len(), 0);
        assert_eq!(f.map.next_symbol(), 1);
        assert_eq!(f.get_ref(), &vec![0usize]);
    }

    #[test]
    fn test_binary_fenwick_paper_example() {
        let mut f = Fenwick::<Cartographer<u8>>::new();
        let sym: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
        let freq: Vec<usize> = vec![1, 1, 1, 4, 3, 5, 2, 3, 6, 5, 4, 1, 1, 9];
        for (x, mut count) in sym.iter().zip(freq) {
            while count > 0 {
                f.update_freq_count(x).unwrap();
                count -= 1;
            }
        }
        let expected: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        assert_eq!(&expected, f.get_ref());
    }
    #[test]
    fn test_binary_fenwick_with_frequencies() {
        let f = get_example_from_paper();
        let expected: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        assert_eq!(&expected, f.get_ref());
    }

    #[test]
    fn test_string_fenwick_with_frequencies() {
        let sym: Vec<String> = vec![
            "Sometimes I've believed as many as six impossible things".to_string(),
            "Who in the world am I? Ah, that's the great puzzle".to_string(),
            "Curiouser and curiouser!".to_string(),
            "How long is forever?".to_string(),
            "Sometimes, just one second".to_string(),
            "Why it's simply impassible!".to_string(),
            "Why, don't you mean impossible?".to_string(),
            "Would you tell me, please, which way I ought to go from here?".to_string(),
            "That depends a good deal on where you want to get to".to_string(),
            "I don't much care where".to_string(),
            "Then it doesn't much matter which way you go".to_string(),
            "So long as I get somewhere".to_string(),
            "Oh, you're sure to do that, if only you walk long enough".to_string(),
            "I knew who I was this morning, but I've changed a few times since then".to_string(),
        ];
        let freq: Vec<usize> = vec![1, 1, 1, 4, 3, 5, 2, 3, 6, 5, 4, 1, 1, 9];
        let f = fenwick_with_string_frequencies(freq, sym);
        let expected: Vec<usize> = vec![0, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10];
        assert_eq!(&expected, f.get_ref());
    }

    #[test]
    fn test_normalization_and_count() {
        let mut f = get_example_from_paper();
        let norm1: Vec<usize> = vec![0, 1, 1, 1, 3, 1, 4, 1, 10, 3, 5, 2, 8, 1, 5]; // sum = 23
        let norm2: Vec<usize> = vec![0, 1, 1, 1, 1, 1, 2, 1, 5, 1, 2, 1, 4, 1, 2]; // sum = 11

        assert_eq!(f.get_total(), 46);
        f.normalize();
        assert_eq!(f.freq, norm1);
        assert_eq!(f.get_total(), 23);
        f.normalize();
        assert_eq!(f.freq, norm2);
        assert_eq!(f.get_total(), 11);
    }

    #[test]
    fn test_backwards() {
        assert_eq!(backward(13), 12);
        assert_eq!(backward(8), 0);
        assert_eq!(backward(2), 0);
        assert_eq!(backward(9), 8);
        assert_eq!(backward(2), 0);
    }

    #[test]
    fn test_forward() {
        assert_eq!(forward(13), 14);
        assert_eq!(forward(14), 16);
        assert_eq!(forward(16), 32);
    }

    #[test]
    fn test_single_frequency_calculation() {
        let f = get_example_from_paper();

        assert_eq!(f.get_h_freq(1), 1);
        assert_eq!(f.get_h_freq(9), 26);
        assert_eq!(f.get_h_freq(7), 17);
        assert_eq!(f.get_h_freq(8), 20);
        assert_eq!(f.get_h_freq(12), 36);
        assert_eq!(f.get_h_freq(14), 46);
    }

    #[test]
    fn test_frequency_tuple_calculation() {
        let f = get_example_from_paper();

        assert_eq!(f.get_freq_bounds(&8), (17, 20, 46));
        assert_eq!(f.get_freq_bounds(&3), (2, 3, 46));
        assert_eq!(f.get_freq_bounds(&10), (26, 31, 46));
        assert_eq!(f.get_freq_bounds(&14), (37, 46, 46));
    }

    #[test]
    fn test_frequency_increment() {
        let mut f = get_example_from_paper();

        assert_eq!(f.get_h_freq(7), 17);
        f.update_freq_count(&7).unwrap();
        f.update_freq_count(&7).unwrap();
        assert_eq!(f.get_h_freq(7), 19);
        f.update_freq_count(&3).unwrap();
        f.update_freq_count(&8).unwrap();
        f.update_freq_count(&12).unwrap();
        assert_eq!(f.get_h_freq(3), 4);
        assert_eq!(f.get_h_freq(8), 24);
        assert_eq!(f.get_h_freq(12), 41);
    }

    #[test]
    fn test_symbol_recovery() {
        let f = get_example_from_paper();
        assert_eq!(f.get_symbol(28).unwrap(), &9u8);
        assert_eq!(f.get_symbol(5).unwrap(), &3u8);
        assert_eq!(f.get_symbol(13).unwrap(), &5u8);
        assert_eq!(f.get_symbol(36).unwrap(), &12u8);
        assert_eq!(f.get_symbol(40).unwrap(), &13u8);
        assert_eq!(f.get_symbol(41).unwrap(), &13u8);
        assert_eq!(f.get_symbol(17).unwrap(), &7u8);
        assert_eq!(f.get_symbol(18).unwrap(), &7u8);
        assert_eq!(f.get_symbol(19).unwrap(), &7u8);
    }

    #[test]
    fn test_feeder() {
        let data: Vec<u8> = vec![
            1, 2, 3, 4, 4, 4, 4, 5, 5, 5, 6, 6, 6, 6, 6, 7, 7, 8, 8, 8, 9, 9, 9, 9, 9, 9, 10, 10,
            10, 10, 10, 11, 11, 11, 11, 12, 13, 14, 14, 14, 14, 14, 14, 14, 14, 14,
        ];
        let expected = get_example_from_paper();
        let mut f = Fenwick::<Cartographer<u8>>::new();
        f.feed(&data).unwrap();
        assert_eq!(f.get_ref(), expected.get_ref())
    }
}
