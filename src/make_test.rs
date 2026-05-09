#[macro_export]
macro_rules! test {
    {
        $name:literal $body:block
    } => {
        #[test_case]
        #[allow(unused_imports)]
        fn _test() {
            $crate::_run_test($name, $body);
        }
    };
}
