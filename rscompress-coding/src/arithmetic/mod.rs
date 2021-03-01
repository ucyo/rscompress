trait Context {}

trait Model {
    fn get_current_context(&self) -> Box<dyn Context>;
}

trait Statistics {
    fn get_freq_count(&self, symbol: u8) -> (usize, usize, usize);
    fn update_freq_count(&mut self, symbol: u8);
    fn get_symbol(&self, target: usize) -> u8;
    fn get_total(&self) -> usize;
    fn _normalize(&mut self);
}

trait AriCoder {
    fn encode_symbol(&self, lower: usize, higher: usize, total: usize);
    fn decode_symbol(&self, total: usize) -> u8;
    fn decode_update(&mut self, symbol: u8);
    fn _normalize(&mut self);
}
