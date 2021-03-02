trait Context {}

trait Model {
    fn get_current_context(&self) -> Box<dyn Context>;
}

mod fenwick;
trait Statistics {
    fn get_freq_bounds(&self, symbol: u8) -> (usize, usize, usize);
    fn update_freq_count(&mut self, symbol: u8);
    fn get_symbol(&self, target: usize) -> usize;
    fn get_total(&self) -> usize;
}

trait AriCoder {
    fn encode_symbol(&self, low: usize, high: usize, total: usize);
    fn decode_symbol(&self, total: usize) -> u8;
    fn decode_update(&mut self, symbol: u8);
}
