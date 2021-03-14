//! Arithemtic Coding
//!
//! # Inner Workings
//! An ideal Arithmetic Coder can compress every arbitrary input to a single real number.
//! All that is needed for this is the probability distribution of the symbols.
//! Lets say we want to code an input given by the alphabet `a = {a, b, c, __ }` with
//! `__` representing empty space and the probability distribution `a = 0.1, b = 0.2, c = 0.3, __ = 0.4`.
//! Any input of arbitary length from this alphabet can be mapped to a single real value between 0 and 1.
//! This works with recursively splitting the value range based on the probability distribution.
//! Here, for the first symbol of the input the ranges are `a = [0.0;0.1), b = [0.1;0.3), c = [0.3;0.6), __ = [0.6;1.0)`.
//! If the coded value is within this range we immediately know the first symbol of the original input e.g. 0.49282 > `c`
//! or 0.012 > `a`. If this is done recursively, then any input can be represented in a single real value.
//! Unfortunately there is one pitfall: Infinite precision is needed for coding input of arbitrary length.
//!
//! # Implementation details
//! The core component of the arithmetic coder is the statistics module responsible for calculating the probability distribution.
//! The two other components are the context model and the coder. The context model reads the input and delivers the context to the statistics model.
//! The statistics model reads in the symbol, updates the probability model and returns the probability for the current symbol.
//! The coder model actually encodes the symbol.
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
mod fenwick;
mod range;

pub type FenwickStatistics<T> = fenwick::Fenwick<fenwick::map::Cartographer<T>>;

trait Context {}

trait Model {
    fn get_current_context(&self) -> Box<dyn Context>;
}

pub trait Statistics {
    type Symbol;
    fn get_freq_bounds(&self, symbol: &Self::Symbol) -> (usize, usize, usize);
    fn update_freq_count(&mut self, symbol: &Self::Symbol) -> Result<(), StatisticsError>;
    fn get_symbol(&self, target: usize) -> Result<&Self::Symbol, StatisticsError>;
    fn get_total(&self) -> usize;
    fn feed(&mut self, data: &[Self::Symbol]) -> Result<(), StatisticsError>;
}

#[derive(Debug)]
pub enum StatisticsError {
    /// Symbol not found in the statistics
    UnknownSymbolError,
    /// Error while updating the statistics
    UpdateError,
}

impl Display for StatisticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatisticsError::UnknownSymbolError => write!(f, "Unknown Symbol"),
            StatisticsError::UpdateError => write!(f, "Could not update Symbol"),
        }
    }
}

impl Error for StatisticsError {}

impl From<fenwick::map::MapError> for StatisticsError {
    fn from(f: fenwick::map::MapError) -> Self {
        match f {
            fenwick::map::MapError::UnknownIndexError => StatisticsError::UpdateError,
            fenwick::map::MapError::UnknownSymbolError => StatisticsError::UnknownSymbolError,
        }
    }
}

trait AriCoder {
    fn encode_symbol(&self, low: usize, high: usize, total: usize);
    fn decode_symbol(&self, total: usize) -> u8;
    fn decode_update(&mut self, symbol: u8);
}
