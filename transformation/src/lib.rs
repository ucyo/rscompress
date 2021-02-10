mod runlength;

pub trait Transform {
    fn transform(&mut self, source: &[u8]) -> Option<Vec<u8>>;
    fn reverse(&mut self, source: &[u8]) -> Option<Vec<u8>>;
}

#[macro_export]
macro_rules! test_transform {
    ($func_name:ident, $method:ident, $input:expr, $expected:expr) => {
        #[test]
        fn $func_name() {
            let mut rl = $method::new();
            let result = rl.transform(&$input).unwrap();

            assert_eq!(result, $expected)
        }
    };
}
