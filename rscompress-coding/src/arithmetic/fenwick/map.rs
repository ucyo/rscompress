use std::collections::HashMap;

/// Mapping of arbitary elements to index position for frequency counts
pub type Mapping<T> = HashMap<T, usize>;

/// Trait for defining the mapping strategy from symbols to freq counts
///
/// The Fenwick Tree saves each symbol in an array.
/// The index position of the symbol is not changed.
/// For character-based symbols the number of symbols is pre-defined i.e. 256.
/// But for word-based symbols the alphabet can of arbitary length.
/// Therefore a mapping is needed from word to symbol index.
/// This trait represents this mapping.
/// The associated type defines the symbol type and can be `u8` (character-based),
/// `String` or `Vec<u8>` (word-based), or anything else.
pub trait Map: Default {
    type Input;
    /// Create new mapping
    fn new() -> Self;
    /// Get index for Symbol
    fn get_index_of(&self, symbol: &Self::Input) -> Option<usize>;
    /// Install a symbol by associating an index position with the symbol
    fn install(&mut self, symbol: &Self::Input) -> usize;
    /// Get inner mapping as a reference
    fn get_ref(&self) -> &Mapping<Self::Input>;
    /// Get the number of elements being mapped
    fn alphabet_size(&self) -> usize;
    /// Get Symbol at index position
    fn get_symbol_at(&self, ix: usize) -> &Self::Input;
}

/// Maps arbitary alphabets to usize and back
///
/// The Cartographer maps arbitary alphabets to usize integers.
/// This way one Fenix Tree implementatino can be used for byte-/character-based,
/// word-based, or any other type-based alphabets.
#[derive(Debug)]
pub struct Cartographer<T> {
    next_symbol: usize,
    map: Mapping<T>
}

impl Default for Cartographer<u8> {
    fn default() -> Self {
        Cartographer { next_symbol: 1, map: Mapping::<u8>::new()}
    }
}

impl Map for Cartographer<u8> {
    type Input = u8;

    fn new() -> Self {
        Default::default()
    }
    fn get_index_of(&self, symbol: &Self::Input) -> Option<usize> {
        Some(*self.map.get(symbol).unwrap())
    }
    fn install(&mut self, symbol: &Self::Input) -> usize {
        assert!(self.map.get(symbol).is_none());
        self.map.insert(*symbol, self.next_symbol);
        let result = self.next_symbol;
        self.next_symbol += 1;
        result
    }
    fn get_ref(&self) -> &Mapping<Self::Input> {
        &self.map
    }
    fn alphabet_size(&self) -> usize {
        self.map.len()
    }
    fn get_symbol_at(&self, ix: usize) -> &Self::Input {
        let result= self.map.iter()
        .find_map(|(key, &val)| if val == ix { Some(key) } else { None });
        result.unwrap()
    }
}


impl Default for Cartographer<Vec<u8>> {
    fn default() -> Self {
        Cartographer { next_symbol: 1, map: Mapping::<Vec<u8>>::new()}
    }
}

impl Map for Cartographer<Vec<u8>> {
    type Input = Vec<u8>;

    fn new() -> Self {
        Default::default()
    }
    fn get_index_of(&self, symbol: &Self::Input) -> Option<usize> {
        Some(*self.map.get(symbol).unwrap())
    }
    fn install(&mut self, symbol: &Self::Input) -> usize {
        assert!(self.map.get(symbol).is_none());
        let s = symbol.clone(); // TODO: Due to arg taking a reference, this cloning is needed
        self.map.insert(s, self.next_symbol);
        let result = self.next_symbol;
        self.next_symbol += 1;
        result
    }
    fn get_ref(&self) -> &Mapping<Self::Input> {
        &self.map
    }
    fn alphabet_size(&self) -> usize {
        self.map.len()
    }
    fn get_symbol_at(&self, ix: usize) -> &Self::Input {
        let result= self.map.iter()
        .find_map(|(key, &val)| if val == ix { Some(key) } else { None });
        result.unwrap()
    }
}

impl Default for Cartographer<String> {
    fn default() -> Self {
        Cartographer { next_symbol: 1, map: Mapping::<String>::new()}
    }
}

impl Map for Cartographer<String> {
    type Input = String;

    fn new() -> Self {
        Default::default()
    }
    fn get_index_of(&self, symbol: &Self::Input) -> Option<usize> {
        Some(*self.map.get(symbol).unwrap())
    }
    fn install(&mut self, symbol: &Self::Input) -> usize {
        assert!(self.map.get(symbol).is_none());
        let s = symbol.clone().to_string();  // TODO: Due to arg taking a reference, this cloning is needed
        self.map.insert(s, self.next_symbol);
        let result = self.next_symbol;
        self.next_symbol += 1;
        result
    }
    fn get_ref(&self) -> &Mapping<Self::Input> {
        &self.map
    }
    fn alphabet_size(&self) -> usize {
        self.map.len()
    }
    fn get_symbol_at(&self, ix: usize) -> &Self::Input {
        let result= self.map.iter()
        .find_map(|(key, &val)| if val == ix { Some(key) } else { None });
        result.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{rngs::OsRng, RngCore};
    use std::collections::HashSet;

    #[test]
    fn test_binary_cartographer_init() {
        let bcart = Cartographer::<u8>::new();

        assert_eq!(bcart.alphabet_size(), 0);
        assert_eq!(bcart.next_symbol, 1);
    }

    #[test]
    fn test_vec_cartographer_init() {
        let bcart = Cartographer::<Vec<u8>>::new();

        assert_eq!(bcart.alphabet_size(), 0);
        assert_eq!(bcart.next_symbol, 1);
    }

    #[test]
    fn test_string_cartographer_init() {
        let bcart = Cartographer::<String>::new();

        assert_eq!(bcart.alphabet_size(), 0);
        assert_eq!(bcart.next_symbol, 1);
    }

    #[test]
    fn test_binary_cartographer_install_symbols_linear() {
        let mut bcart = Cartographer::<u8>::new();
        for symbol in 0..25u8 {
            bcart.install(&symbol);
        }
        for symbol in 0..25u8 {
            assert_eq!(bcart.get_index_of(&symbol).unwrap(), symbol as usize + 1)
        }
    }

    #[test]
    fn test_vec_cartographer_install_symbols_linear() {
        let mut bcart = Cartographer::<Vec<u8>>::new();
        for symbol in 0..25u8 {
            bcart.install(&vec![symbol]);
        }
        for symbol in 0..25u8 {
            assert_eq!(bcart.get_index_of(&vec![symbol]).unwrap(), symbol as usize + 1)
        }
    }

    #[test]
    fn test_string_cartographer_install_symbols_linear() {
        let mut bcart = Cartographer::<String>::new();

        for symbol in 0..25u8 {
            let k  = format!("{}", symbol);
            bcart.install(&k);
        }
        for symbol in 0..25u8 {
            let k  = format!("{}", symbol);
            assert_eq!(bcart.get_index_of(&k).unwrap(), symbol as usize + 1)
        }
    }

    #[test]
    fn test_binary_cartographer_install_symbols_random() {
        let mut bcart = Cartographer::<u8>::new();
        let mut symbols = vec![0u8; 1000];
        OsRng.fill_bytes(&mut symbols);
        // Remove duplicates
        symbols = symbols.into_iter().collect::<HashSet<_>>().into_iter().collect();

        for symbol in symbols.iter() {
            bcart.install(symbol);
        }
        for symbol in symbols.iter().enumerate() {
            assert_eq!(bcart.get_index_of(&symbol.1).unwrap(), symbol.0 + 1)
        }
        assert_eq!(bcart.alphabet_size(), symbols.len());
        assert_eq!(*bcart.get_symbol_at(4), symbols[3]);
    }

    #[test]
    fn test_vec_cartographer_install_symbols_random() {
        let mut bcart = Cartographer::<Vec<u8>>::new();

        for num in 0..10 {
            let mut symbols = vec![0u8; 1000];
            OsRng.fill_bytes(&mut symbols);
            symbols = symbols.into_iter().collect::<HashSet<_>>().into_iter().collect();
            bcart.install(&symbols);
            assert_eq!(bcart.get_index_of(&symbols).unwrap(), num + 1);
            assert_eq!(bcart.alphabet_size(), num + 1);
        }
    }

    #[test]
    fn test_string_cartographer_install_symbols_random() {
        let mut bcart = Cartographer::<String>::new();
        let mut symbols = vec![0u8; 1000];
        OsRng.fill_bytes(&mut symbols);
        symbols = symbols.into_iter().collect::<HashSet<_>>().into_iter().collect();
        let symbols: Vec<String> = symbols.iter().map(|s| format!("{}", s)).collect();

        for symbol in symbols.iter() {
            bcart.install(symbol);
        }
        for symbol in symbols.iter().enumerate() {
            assert_eq!(bcart.get_index_of(&symbol.1).unwrap(), symbol.0 + 1)
        }
        assert_eq!(bcart.alphabet_size(), symbols.len());
        assert_eq!(*bcart.get_symbol_at(4), symbols[3]);
    }
}
