use std::fmt;
use std::fmt::{Debug, Display};
use std::error::Error;

trait Context {}

trait Model {
    fn get_current_context(&self) -> Box<dyn Context>;
}

mod fenwick;
trait Statistics {
    type Symbol;
    fn get_freq_bounds(&self, symbol: &Self::Symbol) -> (usize, usize, usize);
    fn update_freq_count(&mut self, symbol: &Self::Symbol);
    fn get_symbol(&self, target: usize) -> &Self::Symbol;
    fn get_total(&self) -> usize;
    fn feed(&mut self, data: &[Self::Symbol]);

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
