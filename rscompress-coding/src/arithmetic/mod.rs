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
}

trait AriCoder {
    fn encode_symbol(&self, low: usize, high: usize, total: usize);
    fn decode_symbol(&self, total: usize) -> u8;
    fn decode_update(&mut self, symbol: u8);
}
