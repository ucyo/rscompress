use crate::arithmetic::Statistics;

#[derive(Debug)]
struct Fenwick {
    freq: Vec<usize>,
    inc: usize,
}

impl Fenwick {
    pub fn new() -> Self {
        Fenwick {
            freq: Vec::<usize>::new(),
            inc: 1,
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Fenwick {
            freq: Vec::<usize>::with_capacity(capacity),
            ..Default::default()
        }
    }
    pub fn with_frequencies(frequencies: Vec<usize>) -> Self {
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
}

impl Default for Fenwick {
    fn default() -> Self {
        Fenwick::new()
    }
}

impl Statistics for Fenwick {
    fn get_total(&self) -> usize {
        self.freq.iter().sum()
    }
    fn get_symbol(&self, target: usize) -> usize {
        todo!()
    }
    fn update_freq_count(&mut self, symbol: u8) {
        todo!()
    }
    fn get_freq_count(&self, symbol: u8) -> (usize, usize, usize) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let frequencies: Vec<usize> = vec![1, 1, 2, 1, 7, 3, 8, 2, 20, 6, 11, 4, 16, 1, 10]; // sum = 93
        let result: Vec<usize> = vec![1, 1, 1, 1, 3, 1, 4, 1, 10, 3, 5, 2, 8, 1, 5]; // sum = 47
        let mut c = Fenwick::with_frequencies(frequencies);

        assert_eq!(c.get_total(), 93);
        c.normalize();
        assert_eq!(c.freq, result);
        assert_eq!(c.get_total(), 47);
    }
}
